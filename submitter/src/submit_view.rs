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
            let api = api.lock().unwrap();
            let rx = api.post_solution(state.problem.clone(), state.solution.clone(), captcha);
            gtk::timeout_add(100, move || {
                if let Ok(res) = rx.try_recv() {
                    println!("{:?}", res);
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
