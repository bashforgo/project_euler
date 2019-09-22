use reqwest::header;
use std::{sync::mpsc, thread};

use super::*;

pub enum LoginResult {
    Fail,
    Success,
    Unknown,
}

impl API {
    pub fn login(
        &self,
        username: String,
        password: String,
        captcha: String,
    ) -> mpsc::Receiver<Option<LoginResult>> {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let req = {
                let api = get_api();
                let api = api.lock().unwrap();

                let req = api.client.post(make_url!("/sign_in")).form(&[
                    ("username", username.as_str()),
                    ("password", password.as_str()),
                    ("captcha", captcha.as_str()),
                    ("remember_me", "1"),
                    ("sign_in", "Sign+In"),
                ]);
                api.use_session(req)
            };

            let res = req
                .send()
                .map(|res| {
                    use LoginResult::*;

                    if res.status() == 302 {
                        let headers = res.headers();

                        let has_location = headers.contains_key(header::LOCATION);

                        if has_location {
                            let location = &headers[header::LOCATION];

                            if location == "archives" {
                                let cookie = res.cookies().next().unwrap();
                                let sess = cookie.value().to_string();
                                let written = SessionStorage::save(sess);
                                if written.is_none() {
                                    eprintln!("couldn't store session");
                                }

                                Success
                            } else if location == "sign_in" {
                                Fail
                            } else {
                                Unknown
                            }
                        } else {
                            Unknown
                        }
                    } else {
                        Unknown
                    }
                })
                .ok();

            tx.send(res).unwrap();
        });

        rx
    }
}
