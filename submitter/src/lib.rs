#[macro_use]
extern crate lazy_static;

mod api;
mod views;

mod app;
mod captcha;
mod router;

use app::App;

pub fn run(problem: &str, solution: &str) -> App {
    App::new(problem, solution)
}
