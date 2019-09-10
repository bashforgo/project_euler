use reqwest::Client;
use std::{
    io::Read,
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct API {
    pub session: Option<String>,
    client: Client,
}

impl API {
    pub fn new() -> API {
        let client = Client::new();
        API {
            session: None,
            client,
        }
    }

    pub fn get_captcha(&self) -> mpsc::Receiver<Option<Box<dyn Read + Send>>> {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let api = get_api();
            let read = api
                .lock()
                .map_err(|_| "can't get api")
                .and_then(|api| {
                    api.client
                        .get("https://projecteuler.net/captcha/show_captcha.php")
                        .send()
                        .map_err(|_| "can't download captcha")
                })
                .map(|res| Box::new(res) as Box<dyn Read + Send>)
                .ok();

            tx.send(read).unwrap();
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
