use project_euler::divisors;

type Size = u128;

pub fn solve() -> String {
    (2..10000)
        .filter(|&a| {
            let b = divisors::sum_of_proper_divisors(a);
            if a == b {
                return false;
            }
            let a_ = divisors::sum_of_proper_divisors(b);
            a == a_
        })
        .sum::<Size>()
        .to_string()
}
