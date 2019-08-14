mod app;
mod captcha;
mod router;
mod submit_view;

use app::App;

pub fn run(problem: &str, solution: &str) -> App {
    App::new(problem, solution)
}
