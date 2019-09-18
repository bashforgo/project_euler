use gtk::prelude::*;
use std::rc::Rc;

use crate::{app::State, captcha::Captcha};

#[derive(Clone)]
pub struct LoginView {
    pub container: gtk::Box,
    username: gtk::Entry,
    password: gtk::Entry,
    captcha: Captcha,
    state: Rc<State>,
}
struct Disconnected(LoginView);

impl LoginView {
    pub fn new(state: Rc<State>) -> LoginView {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 8);
        container.set_property("vexpand", &true).unwrap();
        container
            .set_property("valign", &gtk::Align::Center)
            .unwrap();

        let label = gtk::Label::new(Some("login to project euler"));
        container.add(&label);

        let username = gtk::Entry::new();
        username.set_placeholder_text(Some("username"));
        container.add(&username);

        let password = gtk::Entry::new();
        password.set_placeholder_text(Some("password"));
        password.set_visibility(false);
        container.add(&password);

        let captcha = Captcha::create();
        container.add(&captcha.container);

        let disconnected = Disconnected(LoginView {
            container,
            username,
            password,
            captcha,
            state,
        });

        disconnected.connect()
    }

    fn on_submit(&self) {
        let username = self.username.get_buffer().get_text();
        let password = self.password.get_buffer().get_text();
        let captcha = self.captcha.get_text();
        println!("username={} password={} captcha={}", username, password, captcha);
    }

    pub fn on_switch_to(&self) {
        self.captcha.get_new();
    }
}

impl Disconnected {
    fn connect(self) -> LoginView {
        let owned = self.0;

        {
            let cloned = owned.clone();
            owned.username.connect_activate(move |_| {
                cloned.on_submit();
            });
        }

        {
            let cloned = owned.clone();
            owned.password.connect_activate(move |_| {
                cloned.on_submit();
            });
        }

        {
            let cloned = owned.clone();
            owned.captcha.connect(move |_| {
                cloned.on_submit();
            });
        }

        owned
    }
}
