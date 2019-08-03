use project_euler::primes::primes;

pub fn solve() -> String {
    primes()
        .take_while(|n| *n < 2_000_000)
        .sum::<u128>()
        .to_string()
}
