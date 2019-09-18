use crate::app::State;
use gtk::prelude::*;
use std::rc::Rc;

use crate::status_view::{self, StatusView};
use crate::submit_view::SubmitView;

pub enum View {
    Submit,
    Status(status_view::Message),
}

pub struct Router {
    pub container: gtk::Stack,
    pub submit_view: SubmitView,
    pub status_view: StatusView,
}

impl Router {
    pub fn new(state: Rc<State>) -> Router {
        let container = gtk::Stack::new();
        let submit_view = SubmitView::new(Rc::clone(&state));
        let status_view = StatusView::new(Rc::clone(&state));

        container.add(&submit_view.container);
        container.add(&status_view.container);

        Router {
            container,
            submit_view,
            status_view,
        }
    }

    pub fn switch_to(&self, view: View) {
        match view {
            View::Submit => {
                self.submit_view.on_switch_to();
                self.container
                    .set_visible_child(&self.submit_view.container);
            }
            View::Status(m) => {
                self.status_view.on_switch_to(m);
                self.container
                    .set_visible_child(&self.status_view.container);
            }
        }
    }
}
