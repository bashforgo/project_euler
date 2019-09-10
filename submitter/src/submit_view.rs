use gtk::prelude::*;
use std::rc::Rc;

use crate::{api::get_api, app::State, captcha::Captcha};

pub struct SubmitView {
    pub container: gtk::Box,
    pub captcha: Captcha,
}

impl SubmitView {
    pub fn new(state: Rc<State>) -> SubmitView {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 8);

        let captcha = Captcha::create().connect(move |captcha| {
            let api = get_api();
            api.lock()
                .map_err(|_| "can't get api")
                .and_then(|api| {
                    api.post_solution(state.problem.clone(), state.solution.clone(), captcha);
                    Ok(())
                })
                .unwrap();
        });
        container.add(&captcha.container);

        SubmitView { container, captcha }
    }
}
