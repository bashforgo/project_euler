use gio::prelude::*;
use gtk::prelude::*;
use std::{rc::Rc, sync::mpsc};

use crate::{
    api::get_api,
    router::{self, Router},
    status_view,
};

pub enum Message {
    SubmitView,
    StatusView(status_view::Message),
    Quit,
}

pub struct State {
    pub problem: String,
    pub solution: String,
    pub dispatch: mpsc::Sender<Message>,
    message_bus: mpsc::Receiver<Message>,
}

impl State {
    pub fn new(problem: String, solution: String) -> State {
        let (dispatch, message_bus) = mpsc::channel();

        State {
            problem,
            solution,
            dispatch,
            message_bus,
        }
    }
}

pub struct UI {
    pub container: gtk::Box,
    pub router: Router,
}

impl UI {
    pub fn new(state: Rc<State>) -> UI {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 8);
        let router = Router::new(Rc::clone(&state));

        container.add(&router.container);

        UI { container, router }
    }
}

pub struct App {
    pub application: gtk::Application,
    pub ui: Rc<UI>,
    pub state: Rc<State>,
}

impl App {
    pub fn new(problem: &str, solution: &str) -> App {
        let application =
            gtk::Application::new(Some("hu.devo.project-euler.submitter"), Default::default())
                .expect("failed to initialize GTK application");

        let state = Rc::new(State::new(problem.into(), solution.into()));
        let ui = Rc::new(UI::new(Rc::clone(&state)));

        App {
            application,
            ui,
            state,
        }
    }

    pub fn connect(&self) {
        let ui = Rc::clone(&self.ui);
        self.application.connect_activate(move |application| {
            let window = gtk::ApplicationWindow::new(application);
            window.set_title("submitter");
            window.set_position(gtk::WindowPosition::Center);
            window.set_size_request(400, 300);

            window.add(&ui.container);

            window.show_all();
            window.present();
        });

        let state = Rc::clone(&self.state);
        {
            let has_session = {
                let api = get_api();
                let api = api.lock().unwrap();
                api.has_session()
            };

            if has_session {
                state.dispatch.send(Message::SubmitView).unwrap();
            } else {
                state
                    .dispatch
                    .send(Message::StatusView(status_view::Message::Unrecoverable(
                        "unimplemented".into(),
                    )))
                    .unwrap();
            }
        }

        let ui = Rc::clone(&self.ui);
        let state = Rc::clone(&self.state);
        let application = self.application.clone();
        gtk::timeout_add(100, move || {
            let message_bus = &state.message_bus;
            if let Ok(message) = message_bus.try_recv() {
                use Message::*;

                match message {
                    SubmitView => {
                        ui.router.switch_to(router::View::Submit);
                    }
                    StatusView(m) => {
                        ui.router.switch_to(router::View::Status(m));
                    }
                    Quit => {
                        application.quit();
                    }
                }
            }

            gtk::Continue(true)
        });

        self.application.run(&[]);
    }
}
