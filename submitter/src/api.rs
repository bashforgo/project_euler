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
            let api = api.lock().unwrap();
            let res = api
                .client
                .get("https://projecteuler.net/captcha/show_captcha.php")
                .send()
                .unwrap();
            let read: Box<dyn Read + Send> = Box::new(res);

            tx.send(Some(read));
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
