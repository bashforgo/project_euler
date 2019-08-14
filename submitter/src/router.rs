use gtk::prelude::*;

use crate::submit_view::SubmitView;

pub enum View {
    Submit,
}

pub struct Router {
    pub container: gtk::Stack,
    pub submit_view: SubmitView,
}

impl Default for Router {
    fn default() -> Router {
        let container = gtk::Stack::new();
        let submit_view = SubmitView::default();

        container.add(&submit_view.container);

        let router = Router {
            container,
            submit_view,
        };

        router.switch_to(View::Submit);

        router
    }
}

impl Router {
    pub fn new() -> Router {
        Router::default()
    }

    pub fn switch_to(&self, view: View) {
        match view {
            View::Submit => {
                self.container
                    .set_visible_child(&self.submit_view.container);
            }
        }
    }
}
