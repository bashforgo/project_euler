use std::{sync::mpsc, thread};

use super::*;

pub enum PostSolutionResult {
    WrongCaptcha,
    BadSolution,
    Success,
    Unknown,
}

impl API {
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

                let req = api.client.post(make_url!("/problem={}", problem)).form(&[
                    (format!("guess_{}", problem).as_str(), solution.as_str()),
                    ("captcha", captcha.as_str()),
                ]);
                api.use_session(req)
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
