use project_euler::primes::primes;

pub fn solve() -> String {
    primes().skip(10000).take(1).sum::<i32>().to_string()
}
