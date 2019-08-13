mod app;
mod router;

use app::App;

pub fn run(problem: &str, solution: &str) -> App {
    App::new(problem, solution)
}
