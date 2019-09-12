use gtk::prelude::*;
use std::rc::Rc;

use crate::app::{self, State};

pub enum Message {
    Success(String),
    Recoverable(String),
    Unrecoverable(String),
}

pub struct StatusView {
    pub container: gtk::Stack,
    pub success: gtk::Box,
    pub success_label: gtk::Label,
    pub recoverable: gtk::Box,
    pub recoverable_label: gtk::Label,
    pub unrecoverable: gtk::Box,
    pub unrecoverable_label: gtk::Label,
}

impl StatusView {
    pub fn new(state: Rc<State>) -> StatusView {
        let container = gtk::Stack::new();

        let success = gtk::Box::new(gtk::Orientation::Vertical, 8);
        success.set_property("vexpand", &true).unwrap();
        success.set_property("valign", &gtk::Align::Center).unwrap();

        let success_label = gtk::Label::new(None);
        success.add(&success_label);
        {
            let success_actions = gtk::Box::new(gtk::Orientation::Horizontal, 8);
            success_actions
                .set_property("halign", &gtk::Align::Center)
                .unwrap();
            success.add(&success_actions);

            {
                let quit_button = gtk::Button::new();
                quit_button.set_label("quit");
                let dispatch = state.dispatch.clone();
                quit_button.connect_clicked(move |_| {
                    dispatch.send(app::Message::Quit).unwrap();
                });
                success_actions.add(&quit_button);
            }
        }
        container.add(&success);

        let recoverable = gtk::Box::new(gtk::Orientation::Vertical, 8);
        recoverable.set_property("vexpand", &true).unwrap();
        recoverable
            .set_property("valign", &gtk::Align::Center)
            .unwrap();

        let recoverable_label = gtk::Label::new(None);
        recoverable.add(&recoverable_label);
        {
            let recoverable_actions = gtk::Box::new(gtk::Orientation::Horizontal, 8);
            recoverable_actions
                .set_property("halign", &gtk::Align::Center)
                .unwrap();
            recoverable.add(&recoverable_actions);

            {
                let retry_button = gtk::Button::new();
                retry_button.set_label("retry");
                let dispatch = state.dispatch.clone();
                retry_button.connect_clicked(move |_| {
                    dispatch.send(app::Message::SubmitView).unwrap();
                });
                recoverable_actions.add(&retry_button);
            }

            {
                let quit_button = gtk::Button::new();
                quit_button.set_label("quit");
                let dispatch = state.dispatch.clone();
                quit_button.connect_clicked(move |_| {
                    dispatch.send(app::Message::Quit).unwrap();
                });
                recoverable_actions.add(&quit_button);
            }
        }
        container.add(&recoverable);

        let unrecoverable = gtk::Box::new(gtk::Orientation::Vertical, 8);
        unrecoverable.set_property("vexpand", &true).unwrap();
        unrecoverable
            .set_property("valign", &gtk::Align::Center)
            .unwrap();

        let unrecoverable_label = gtk::Label::new(None);
        unrecoverable.add(&unrecoverable_label);
        {
            let unrecoverable_actions = gtk::Box::new(gtk::Orientation::Horizontal, 8);
            unrecoverable_actions
                .set_property("halign", &gtk::Align::Center)
                .unwrap();
            unrecoverable.add(&unrecoverable_actions);

            {
                let quit_button = gtk::Button::new();
                quit_button.set_label("quit");
                let dispatch = state.dispatch.clone();
                quit_button.connect_clicked(move |_| {
                    dispatch.send(app::Message::Quit).unwrap();
                });
                unrecoverable_actions.add(&quit_button);
            }
        }
        container.add(&unrecoverable);

        StatusView {
            container,
            success,
            success_label,
            recoverable,
            recoverable_label,
            unrecoverable,
            unrecoverable_label,
        }
    }

    pub fn on_switch_to(&self, message: Message) {
        use Message::*;

        match message {
            Success(label) => {
                self.success_label.set_label(&label);
                self.container.set_visible_child(&self.success);
            }
            Recoverable(label) => {
                self.recoverable_label.set_label(&label);
                self.container.set_visible_child(&self.recoverable);
            }
            Unrecoverable(label) => {
                self.unrecoverable_label.set_label(&label);
                self.container.set_visible_child(&self.unrecoverable);
            }
        };
    }
}
