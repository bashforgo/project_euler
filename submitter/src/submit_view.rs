use gtk::prelude::*;
use std::rc::Rc;

use crate::{
    api::{get_api, PostSolutionResult},
    app::{Message, State},
    captcha::Captcha,
    status_view,
};

pub struct SubmitView {
    pub container: gtk::Box,
    pub captcha: Captcha,
}

impl SubmitView {
    pub fn new(state: Rc<State>) -> SubmitView {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 8);

        let captcha = Captcha::create().connect(move |captcha| {
            let api = get_api();
            let api = api.lock().unwrap();
            let rx = api.post_solution(state.problem.clone(), state.solution.clone(), captcha);

            let state = Rc::clone(&state);
            gtk::timeout_add(100, move || {
                if let Ok(res) = rx.try_recv() {
                    use PostSolutionResult::*;

                    let message = match res {
                        Some(WrongCaptcha) => "incorrect captcha",
                        _ => unreachable!(),
                    }
                    .to_string();

                    state
                        .dispatch
                        .send(Message::StatusView(status_view::Message { message }))
                        .unwrap();
                    gtk::Continue(false)
                } else {
                    gtk::Continue(true)
                }
            });
        });
        container.add(&captcha.container);

        SubmitView { container, captcha }
    }
}
