use project_euler::primes::is_prime;

pub fn solve() -> String {
    let mut max_len = 0;
    let mut pair = None;
    for a in -999..=999 {
        for b in 2..=999 {
            // https://projecteuler.net/thread=27;post=683
            if !is_prime(b as u128) {
                continue;
            }
            for n in 0.. {
                let q = (n * n) + (a * n) + b;
                if q < 0 {
                    break;
                }
                if !is_prime(q as u128) {
                    if n > max_len {
                        max_len = n;
                        pair = Some((a, b));
                    }
                    break;
                }
            }
        }
    }

    let pair = pair.unwrap();
    (pair.0 * pair.1).to_string()
}
