use gtk::prelude::*;
use std::rc::Rc;

use crate::{
    api::{get_api, PostSolutionResult},
    app::{Action, State},
    captcha::Captcha,
    router::View,
    views::status,
};

pub struct SubmitView {
    pub container: gtk::Box,
    pub captcha: Captcha,
}

impl SubmitView {
    pub fn new(state: Rc<State>) -> SubmitView {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 8);
        container.set_property("vexpand", &true).unwrap();
        container
            .set_property("valign", &gtk::Align::Center)
            .unwrap();

        let label = format!("submitting problem {}: {}", state.problem, state.solution);
        let label = gtk::Label::new(Some(&label));
        container.add(&label);

        let captcha = Captcha::create();
        captcha.connect(move |captcha| {
            let api = get_api();
            let api = api.lock().unwrap();
            let rx = api.post_solution(state.problem.clone(), state.solution.clone(), captcha);

            let state = Rc::clone(&state);
            gtk::timeout_add(100, move || {
                if let Ok(res) = rx.try_recv() {
                    use status::Message;
                    use PostSolutionResult::*;

                    let here = Box::new(View::Submit);
                    let message = match res {
                        Some(WrongCaptcha) => Message::Recoverable("wrong captcha".into(), here),
                        Some(BadSolution) => Message::Unrecoverable("wrong solution".into()),
                        Some(Success) => Message::Success("solution submitted".into()),
                        Some(Unknown) => Message::Recoverable("unknown error".into(), here),
                        None => Message::Recoverable("network error".into(), here),
                    };

                    state
                        .dispatch
                        .send(Action::SwitchTo(View::Status(message)))
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

    pub fn on_switch_to(&self) {
        self.captcha.get_new();
    }
}
