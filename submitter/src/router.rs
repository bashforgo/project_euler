use std::rc::Rc;
use gtk::prelude::*;

use crate::app::State;
use crate::submit_view::SubmitView;

pub enum View {
    Submit,
}

pub struct Router {
    pub container: gtk::Stack,
    pub submit_view: SubmitView,
}

impl Router {
    pub fn new(state: Rc<State>) -> Router {
        let container = gtk::Stack::new();
        let submit_view = SubmitView::new(state);

        container.add(&submit_view.container);

        let router = Router {
            container,
            submit_view,
        };

        router.switch_to(View::Submit);

        router
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
