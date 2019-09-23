use std::env;
use std::process;

mod problems;

#[cfg(feature = "submit")]
fn use_solution(problem: &str, solution: &str) {
    submitter::run(problem, &solution).connect();
}

#[cfg(not(feature = "submit"))]
fn use_solution(_problem: &str, solution: &str) {
    println!("{}", solution);
}

fn main() {
    let args = env::args();
    if args.len() < 2 {
        println!("which problem to submit?");
        process::exit(1);
    } else {
        let problem = &args.collect::<Vec<_>>()[1];
        if let Some(solver) = problems::solvers().get(problem) {
            let solution = solver();
            use_solution(problem, &solution);
        } else {
            println!("problem \"{}\" not solved", problem);
            process::exit(1);
        }
    }
}
