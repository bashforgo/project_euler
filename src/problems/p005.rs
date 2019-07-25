const PRIMES: [i32; 8] = [2, 3, 5, 7, 11, 13, 17, 19];

pub fn solve() -> String {
    (1..)
        .into_iter()
        // .filter(|n| (2..20).into_iter().all(|m| n % m == 0))
        .filter(|n| n % 20 == 0)
        .filter(|n| n % 18 == 0)
        .filter(|n| n % 16 == 0)
        .filter(|n| PRIMES.iter().all(|p| n % p == 0))
        .take(1)
        .sum::<i32>()
        .to_string()
}