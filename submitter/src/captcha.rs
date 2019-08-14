use cairo;
use gtk::prelude::*;

const EXAMPLE_IMAGE: &[u8; 4304] = include_bytes!("./example.png");

pub struct Captcha {
    pub container: gtk::Box,
    pub image: gtk::Image,
    pub entry: gtk::Entry,
}
pub struct Disconnected(Captcha);

impl Captcha {
    pub fn create() -> Disconnected {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 8);

        let mut boxed = Box::new(&EXAMPLE_IMAGE[..]);
        let surface = cairo::ImageSurface::create_from_png(&mut boxed).unwrap();
        let image = gtk::Image::new_from_surface(Some(&surface));
        container.add(&image);

        let entry = gtk::Entry::new();
        container.add(&entry);

        Disconnected(Captcha {
            container,
            image,
            entry,
        })
    }
}

impl Disconnected {
    pub fn connect(self) -> Captcha {
        let inner = self.0;
        let buf = inner.entry.get_buffer();

        inner.entry.connect_activate(move |_| {
            println!("value is {:?}", buf.get_text());
        });


        inner
    }
}
