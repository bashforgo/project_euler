use gtk::Application;

mod app;

use app::App;

pub fn run(problem: &str, solution: &str) {
    let application = Application::new(Some("hu.devo.project-euler.submitter"), Default::default())
        .expect("failed to initialize GTK application");

    let app = App::new(application, problem, solution);
    app.init();
}
