use std::env;
use std::process;

mod problems;

fn main() {
    let args = env::args();
    if args.len() < 2 {
        println!("which problem to submit?");
        process::exit(1);
    } else {
        let problem = &args.collect::<Vec<_>>()[1];
        if let Some(solver) = problems::solvers().get(problem) {
            println!("{}", solver());
        } else {
            println!("problem \"{}\" not solved", problem);
            process::exit(1);
        }
    }
}
