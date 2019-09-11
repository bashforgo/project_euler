use gio::prelude::*;
use gtk::prelude::*;
use std::{rc::Rc, sync::mpsc};

use crate::{
    status_view,
    router::{self, Router}};

pub enum Message {
    StatusView(status_view::Message),
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
    pub status: gtk::Label,
}

impl UI {
    pub fn new(state: Rc<State>) -> UI {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 8);
        let router = Router::new(Rc::clone(&state));
        let text = format!(
            "solution for problem {} is {}",
            state.problem, state.solution
        );
        let status = gtk::Label::new(Some(&text));

        container.add(&router.container);
        container.add(&status);

        UI {
            container,
            router,
            status,
        }
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
        let state = Rc::clone(&self.state);
        self.application.connect_activate(move |application| {
            let window = gtk::ApplicationWindow::new(application);
            window.set_title("submitter");
            window.set_position(gtk::WindowPosition::Center);
            window.set_size_request(400, 300);

            let text = format!(
                "solution for problem {} is {}",
                state.problem, state.solution
            );
            ui.status.set_text(&text);
            window.add(&ui.container);

            window.show_all();
            window.present();
        });

        let ui = Rc::clone(&self.ui);
        let state = Rc::clone(&self.state);
        gtk::timeout_add(100, move || {
            let message_bus = &state.message_bus;
            if let Ok(message) = message_bus.try_recv() {
                use Message::*;

                match message {
                    StatusView(m) => {
                        ui.router.switch_to(router::View::Status(m));
                    }
                }
            }

            gtk::Continue(true)
        });

        self.application.run(&[]);
    }
}
