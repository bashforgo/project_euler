use std::{io::Read, sync::mpsc, thread};

use super::*;

impl API {
    pub fn get_captcha(&self) -> mpsc::Receiver<Option<impl Read + Send>> {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let req = {
                let api = get_api();
                let api = api.lock().unwrap();

                let req = api.client.get(make_url!("/captcha/show_captcha.php"));
                api.use_session(req)
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
}
