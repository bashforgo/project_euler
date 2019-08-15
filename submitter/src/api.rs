use std::sync::{Arc, Mutex};

pub struct API {
    pub session: Option<String>,
}

impl API {
    pub fn new() -> API {
        API { session: None }
    }
}

type SharedAPI = Arc<Mutex<API>>;

lazy_static! {
    static ref API_INSTANCE: SharedAPI = Arc::new(Mutex::new(API::new()));
}

pub fn get_api() -> SharedAPI {
    Arc::clone(&API_INSTANCE)
}
