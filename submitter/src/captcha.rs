use gtk::prelude::*;

use crate::api::get_api;

pub struct Captcha {
    pub container: gtk::Box,
    pub image: gtk::Image,
    pub entry: gtk::Entry,
}
pub struct Disconnected(Captcha);

impl Captcha {
    pub fn create() -> Disconnected {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 8);

        let image = gtk::Image::new();
        container.add(&image);

        let entry = gtk::Entry::new();
        container.add(&entry);

        Disconnected(Captcha {
            container,
            image,
            entry,
        })
    }

    pub fn get_new(&self) {
        let api = get_api();
        let api = api.lock().unwrap();
        let rx = api.get_captcha();

        let image = self.image.clone();
        gtk::timeout_add(100, move || {
            if let Ok(read) = rx.try_recv() {
                if let Some(mut read) = read {
                    let surface = cairo::ImageSurface::create_from_png(&mut read).unwrap();
                    image.set_from_surface(Some(&surface));
                }
                gtk::Continue(false)
            } else {
                gtk::Continue(true)
            }
        });
    }
}

impl Disconnected {
    pub fn connect<F: Fn(String) -> () + 'static>(self, on_submit: F) -> Captcha {
        let inner = self.0;

        inner.entry.connect_activate(move |entry| {
            let text = entry.get_buffer().get_text();
            on_submit(text);
        });

        inner
    }
}
