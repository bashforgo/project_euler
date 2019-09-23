use std::collections::HashSet;

use project_euler::divisors;

const MAX: u128 = 28124;

pub fn solve() -> String {
    let abundant_numbers = (2..MAX)
        .map(|n| (n, divisors::sum_of_proper_divisors(n)))
        .filter(|(n, sopd)| sopd > n)
        .map(|t| t.0)
        .collect::<Vec<_>>();

    println!("{}", abundant_numbers.len());

    let mut set = HashSet::new();

    for n in &abundant_numbers {
        for m in &abundant_numbers {
            if m > n {
                break;
            }
            let nm = n + m;
            if nm > MAX {
                break;
            }
            set.insert(nm);
        }
    }

    (1..MAX)
        .filter(|n| !set.contains(n))
        .sum::<u128>()
        .to_string()
}
