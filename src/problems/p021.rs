use project_euler::integer;

type Size = u128;

pub fn solve() -> String {
    (2..10000)
        .filter(|&a| {
            let b = integer::sum_of_proper_divisors(a);
            if a == b {
                return false;
            }
            let a_ = integer::sum_of_proper_divisors(b);
            a == a_
        })
        .sum::<Size>()
        .to_string()
}
