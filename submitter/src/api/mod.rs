use reqwest::{Client, RedirectPolicy};
use std::sync::{Arc, Mutex};

#[macro_use]
mod make_url;

mod get_captcha;
mod is_authenticated;
mod login;
mod post_solution;
mod session;
mod session_storage;

use session_storage::SessionStorage;

pub use login::LoginResult;
pub use post_solution::PostSolutionResult;

pub struct API {
    pub session: Option<String>,
    client: Client,
}

impl API {
    fn new() -> API {
        let client = Client::builder()
            .redirect(RedirectPolicy::none())
            .build()
            .unwrap();

        API {
            session: SessionStorage::get(),
            client,
        }
    }
}

type SharedAPI = Arc<Mutex<API>>;

lazy_static! {
    static ref API_INSTANCE: SharedAPI = Arc::new(Mutex::new(API::new()));
}

pub fn get_api() -> SharedAPI {
    Arc::clone(&API_INSTANCE)
}
