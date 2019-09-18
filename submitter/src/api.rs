use dirs;
use reqwest::{header, Client, RedirectPolicy};
use std::{
    fs,
    io::Read,
    sync::{mpsc, Arc, Mutex},
    thread,
};

const BASE_URL: &str = "https://projecteuler.net";

macro_rules! make_url {
    ($($arg:tt)*) => (&format!("{}{}", BASE_URL, $($arg)*))
}

#[derive(Debug)]
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

fn get_session() -> Option<String> {
    let mut home = dirs::home_dir()?;
    home.push(".project_euler_session");
    fs::read_to_string(home).ok()
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

    pub fn get_captcha(&self) -> mpsc::Receiver<Option<Box<dyn Read + Send>>> {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let api = get_api();
            let read = api
                .lock()
                .map_err(|_| "can't get api")
                .and_then(|api| {
                    let mut req = api.client.get(make_url!("/captcha/show_captcha.php"));

                    if let Some(sess) = &api.session {
                        req = req.header(header::COOKIE, format!("PHPSESSID={}", sess));
                    }

                    req.send().map_err(|_| "can't download captcha")
                })
                .map(|res| Box::new(res) as Box<dyn Read + Send>)
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
            let api = get_api();
            let res = api
                .lock()
                .map_err(|_| "can't get api")
                .and_then(|api| {
                    let mut req = api
                        .client
                        .post(make_url!(format!("/problem={}", problem)))
                        .form(&[
                            (format!("guess_{}", problem).as_str(), solution.as_str()),
                            ("captcha", captcha.as_str()),
                        ]);

                    if let Some(sess) = &api.session {
                        req = req.header(header::COOKIE, format!("PHPSESSID={}", sess));
                    }

                    req.send().map_err(|_| "error posting solution")
                })
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
