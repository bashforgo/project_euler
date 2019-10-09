use project_euler::integer;

pub fn solve() -> String {
    let n = integer::factorial(100);
    n.to_string()
        .drain(..)
        .map(|c| c.to_string().parse::<u16>().unwrap())
        .sum::<u16>()
        .to_string()
}
