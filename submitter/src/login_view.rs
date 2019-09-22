use gtk::prelude::*;
use std::rc::Rc;

use crate::{
    api::{get_api, SignInResult},
    app::{self, State},
    captcha::Captcha,
    router::View,
    status_view,
};

#[derive(Clone)]
pub struct LoginView {
    pub container: gtk::Box,
    label: gtk::Label,
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

        let label = gtk::Label::new(None);
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
            label,
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

        println!(
            "username={} password={} captcha={}",
            username, password, captcha
        );

        if vec![&username, &password, &captcha]
            .iter()
            .any(|s| s.is_empty())
        {
            self.label.get_style_context().add_class("important");
            self.label.set_label("all fields are required");
            return;
        }

        let rx = {
            let api = get_api();
            let api = api.lock().unwrap();

            api.login(username, password, captcha)
        };

        let state = Rc::clone(&self.state);
        gtk::timeout_add(100, move || {
            if let Ok(res) = rx.try_recv() {
                use status_view::Message;
                use SignInResult::*;

                let here = Box::new(View::Login);
                let view = match res {
                    Some(Success) => View::Submit,
                    Some(Fail) => View::Status(Message::Recoverable("login failed".into(), here)),
                    Some(Unknown) => {
                        View::Status(Message::Recoverable("unknown error".into(), here))
                    }
                    None => View::Status(Message::Recoverable("network error".into(), here)),
                };

                state.dispatch.send(app::Message::SwitchTo(view)).unwrap();

                gtk::Continue(false)
            } else {
                gtk::Continue(true)
            }
        });
    }

    pub fn on_switch_to(&self) {
        self.label.get_style_context().remove_class("important");
        self.label.set_label("login to project euler");
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
