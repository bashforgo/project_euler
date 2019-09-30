use num::{pow::Pow, BigUint};
use std::collections::HashSet;

pub fn solve() -> String {
    let mut set = HashSet::new();

    for a in 2_u16..=100 {
        for b in 2_u16..=100 {
            let pow = BigUint::from(a).pow(b);
            set.insert(pow);
        }
    }

    set.len().to_string()
}
