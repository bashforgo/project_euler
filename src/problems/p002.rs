use project_euler::fib::Fib;

pub fn solve() -> String {
    Fib::default()
        .take_while(|f| *f < 4_000_000)
        .filter(|f| f % 2 == 0)
        .sum::<i32>()
        .to_string()
}
