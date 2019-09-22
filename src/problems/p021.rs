use project_euler::factors;

type Size = u128;

pub fn sum_of_divisors(n: Size) -> Size {
    if n == 1 {
        return 1;
    }

    let ps = factors::componentize(n);

    let mut sum = 1;
    for (p, pow) in ps {
        sum *= (0..=pow).map(|i| p.pow(i as u32)).sum::<u128>();
    }

    sum - n
}

pub fn solve() -> String {
    (2..10000)
        .filter(|&a| {
            let b = sum_of_divisors(a);
            if a == b {
                return false;
            }
            let a_ = sum_of_divisors(b);
            a == a_
        })
        .sum::<Size>()
        .to_string()
}
