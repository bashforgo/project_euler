use gtk::prelude::*;

pub struct LoginView {
    pub container: gtk::Box,
}

impl Default for LoginView {
    fn default() -> Self {
        Self::new()
    }
}

impl LoginView {
    pub fn new() -> LoginView {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 8);
        container.set_property("vexpand", &true).unwrap();
        container
            .set_property("valign", &gtk::Align::Center)
            .unwrap();

        let label = gtk::Label::new(Some("login"));

        container.add(&label);

        LoginView { container }
    }
}
