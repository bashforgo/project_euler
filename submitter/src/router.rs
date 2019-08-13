pub struct Router {
    pub container: gtk::Box,
}

impl Router {
    pub fn new() -> Router {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 8);

        Router { container }
    }
}