use std::{sync::mpsc, thread};

use super::*;

impl API {
    pub fn is_authenticated(&self) -> mpsc::Receiver<Option<bool>> {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let req = {
                let api = get_api();
                let api = api.lock().unwrap();

                let req = api.client.get(make_url!("/progress"));
                api.use_session(req)
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
                        let destroyed = SessionStorage::destroy();
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
}
