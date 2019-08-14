use gtk::prelude::*;

use crate::captcha::Captcha;

pub struct SubmitView {
    pub container: gtk::Box,
    pub captcha: Captcha,
}

impl Default for SubmitView {
    fn default() -> SubmitView {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 8);

        let captcha = Captcha::create().connect();
        container.add(&captcha.container);

        SubmitView { container, captcha }
    }
}
