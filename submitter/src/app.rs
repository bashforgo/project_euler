use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label, WindowPosition};

pub struct App {
    application: Application,
    problem: String,
    solution: String,
}

impl App {
    pub fn new(application: Application, problem: &str, solution: &str) -> App {
        App {
            application,
            problem: problem.into(),
            solution: solution.into(),
        }
    }

    pub fn init(&self) {
        let App {
            application,
            problem,
            solution,
        } = &self;
        let problem = problem.clone();
        let solution = solution.clone();

        application.connect_activate(move |application| {
            let window = ApplicationWindow::new(application);
            window.set_title("submitter");
            window.set_position(WindowPosition::Center);
            window.set_size_request(400, 300);

            let text = format!("solution for problem {} is {}", problem, solution);
            let label = Label::new(Some(&text));
            window.add(&label);

            window.show_all();
            window.present();
        });

        application.run(&[]);
    }
}
