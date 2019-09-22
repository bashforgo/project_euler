use dirs;
use reqwest::{header, Client, RedirectPolicy};
use std::{
    fs,
    io::Read,
    path::PathBuf,
    sync::{mpsc, Arc, Mutex},
    thread,
};

const BASE_URL: &str = "https://projecteuler.net";

macro_rules! make_url {
    ($($arg:tt)*) => (&format!("{}{}", BASE_URL, format!($($arg)*)));
}

pub enum SignInResult {
    Fail,
    Success,
    Unknown,
}

pub enum PostSolutionResult {
    WrongCaptcha,
    BadSolution,
    Success,
    Unknown,
}

pub struct API {
    pub session: Option<String>,
    client: Client,
}

fn session_storage() -> Option<PathBuf> {
    let mut home = dirs::home_dir()?;
    home.push(".project_euler_session");
    Some(home)
}

fn get_session() -> Option<String> {
    fs::read_to_string(session_storage()?).ok()
}

fn store_session(sess: String) -> Option<()> {
    fs::write(session_storage()?, sess.as_bytes()).ok()
}

fn destroy_session() -> Option<()> {
    fs::remove_file(session_storage()?).ok()
}

impl API {
    fn new() -> API {
        let client = Client::builder()
            .redirect(RedirectPolicy::none())
            .build()
            .unwrap();

        API {
            session: get_session(),
            client,
        }
    }

    pub fn has_session(&self) -> bool {
        self.session.is_some()
    }

    pub fn is_authenticated(&self) -> mpsc::Receiver<Option<bool>> {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let req = {
                let api = get_api();
                let api = api.lock().unwrap();

                let mut req = api.client.get(make_url!("/progress"));

                if let Some(sess) = &api.session {
                    req = req.header(header::COOKIE, format!("PHPSESSID={}", sess));
                }

                req
            };

            let read = req
                .send()
                .and_then(|mut res| {
                    let text = res.text()?;

                    Ok(!text.contains("Sign In"))
                })
                .map(|is_authenticated| {
                    if !is_authenticated {
                        let api = get_api();
                        let mut api = api.lock().unwrap();

                        api.session.take();
                        let destroyed = destroy_session();
                        if destroyed.is_none() {
                            eprintln!("couldn't destroy session");
                        }
                    }

                    is_authenticated
                })
                .ok();

            tx.send(read).unwrap();
        });

        rx
    }

    pub fn login(
        &self,
        username: String,
        password: String,
        captcha: String,
    ) -> mpsc::Receiver<Option<SignInResult>> {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let req = {
                let api = get_api();
                let api = api.lock().unwrap();

                let mut req = api.client.post(make_url!("/sign_in")).form(&[
                    ("username", username.as_str()),
                    ("password", password.as_str()),
                    ("captcha", captcha.as_str()),
                    ("remember_me", "1"),
                    ("sign_in", "Sign+In"),
                ]);

                if let Some(sess) = &api.session {
                    req = req.header(header::COOKIE, format!("PHPSESSID={}", sess));
                }

                req
            };

            let res = req
                .send()
                .map(|res| {
                    use SignInResult::*;

                    if res.status() == 302 {
                        let headers = res.headers();

                        let has_location = headers.contains_key(header::LOCATION);

                        if has_location {
                            let location = &headers[header::LOCATION];

                            if location == "archives" {

                                let cookie = res.cookies().next().unwrap();
                                let sess = cookie.value().to_string();
                                let written = store_session(sess);
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

    pub fn get_captcha(&self) -> mpsc::Receiver<Option<impl Read + Send>> {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let req = {
                let api = get_api();
                let api = api.lock().unwrap();

                let mut req = api.client.get(make_url!("/captcha/show_captcha.php"));

                if let Some(sess) = &api.session {
                    req = req.header(header::COOKIE, format!("PHPSESSID={}", sess));
                }

                req
            };

            let read = req
                .send()
                .map(|res| {
                    let api = get_api();
                    let mut api = api.lock().unwrap();

                    if !api.has_session() {
                        let mut cookies = res.cookies();
                        let cookie = cookies.next().unwrap();
                        api.session = Some(cookie.value().to_string());
                    }

                    res
                })
                .ok();

            tx.send(read).unwrap();
        });

        rx
    }

    pub fn post_solution(
        &self,
        problem: String,
        solution: String,
        captcha: String,
    ) -> mpsc::Receiver<Option<PostSolutionResult>> {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let req = {
                let api = get_api();
                let api = api.lock().unwrap();

                let mut req = api.client.post(make_url!("/problem={}", problem)).form(&[
                    (format!("guess_{}", problem).as_str(), solution.as_str()),
                    ("captcha", captcha.as_str()),
                ]);

                if let Some(sess) = &api.session {
                    req = req.header(header::COOKIE, format!("PHPSESSID={}", sess));
                }

                req
            };

            let res = req
                .send()
                .map_err(|_| "error posting solution")
                .and_then(|mut res| {
                    use PostSolutionResult::*;

                    let text = res.text().map_err(|_| "can't parse body")?;

                    if res.status() == 302 {
                        Ok(WrongCaptcha)
                    } else if text.contains("answer_wrong.png") {
                        Ok(BadSolution)
                    } else if text.contains("answer_correct.png") {
                        Ok(Success)
                    } else {
                        Ok(Unknown)
                    }
                })
                .ok();

            tx.send(res).unwrap();
        });

        rx
    }
}

type SharedAPI = Arc<Mutex<API>>;

lazy_static! {
    static ref API_INSTANCE: SharedAPI = Arc::new(Mutex::new(API::new()));
}

pub fn get_api() -> SharedAPI {
    Arc::clone(&API_INSTANCE)
}
