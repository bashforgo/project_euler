use gio::prelude::*;
use gtk::prelude::*;
use std::{rc::Rc, sync::mpsc, u32};

use crate::{
    api::get_api,
    router::{Router, View},
};

const APP_CSS: &[u8] = include_bytes!("app.css");

pub enum Action {
    SwitchTo(View),
    Quit,
}

pub struct State {
    pub problem: String,
    pub solution: String,
    pub dispatch: mpsc::Sender<Action>,
    message_bus: mpsc::Receiver<Action>,
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

            let css_provider = gtk::CssProvider::new();
            css_provider.load_from_data(APP_CSS).unwrap();

            let screen = window.get_screen().unwrap();
            gtk::StyleContext::add_provider_for_screen(&screen, &css_provider, u32::MAX);

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

            let is_authenticated = {
                if has_session {
                    let rx = {
                        let api = get_api();
                        let api = api.lock().unwrap();
                        api.is_authenticated()
                    };
                    rx.recv().unwrap().unwrap_or(false)
                } else {
                    false
                }
            };

            if is_authenticated {
                state.dispatch.send(Action::SwitchTo(View::Submit)).unwrap();
            } else {
                state.dispatch.send(Action::SwitchTo(View::Login)).unwrap();
            }
        }

        let ui = Rc::clone(&self.ui);
        let state = Rc::clone(&self.state);
        let application = self.application.clone();
        gtk::timeout_add(100, move || {
            let message_bus = &state.message_bus;
            if let Ok(action) = message_bus.try_recv() {
                use Action::*;

                match action {
                    SwitchTo(view) => {
                        ui.router.switch_to(view);
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
