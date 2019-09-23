use std::collections::HashSet;

use project_euler::divisors;

const MAX: u128 = 28124;

pub fn solve() -> String {
    let mut abundant = HashSet::new();

    let mut sum = 1;
    'outer: for n in 2..MAX {
        let sopd = divisors::sum_of_proper_divisors(n);
        if sopd > n {
            abundant.insert(n);
        }

        for a in &abundant {
            if abundant.contains(&(n - a)) {
                continue 'outer;
            }
        }

        sum += n;
    }

    sum.to_string()
}
