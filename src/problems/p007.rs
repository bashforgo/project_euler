use project_euler::primes::primes;

pub fn solve() -> String {
    primes().skip(10000).take(1).sum::<u128>().to_string()
}
