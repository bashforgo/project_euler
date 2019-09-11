use gtk::prelude::*;

pub struct Message {
    pub message: String,
}

pub struct StatusView {
    pub container: gtk::Box,
    pub label: gtk::Label,
}

impl StatusView {
    pub fn new() -> StatusView {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 8);

        let label = gtk::Label::new(None);
        container.add(&label);

        StatusView { container, label }
    }

    pub fn on_switch_to(&self, message: Message) {
        self.label.set_label(message.message.as_str());
    }
}
