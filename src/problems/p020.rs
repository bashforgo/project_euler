use num::BigInt;

fn factorial(mut n: i32) -> BigInt {
    let mut f = BigInt::from(n);
    while n > 1 {
        f *= n;
        n -= 1;
    }
    f
}

pub fn solve() -> String {
    let n = factorial(100);
    n.to_string()
        .drain(..)
        .map(|c| c.to_string().parse::<u16>().unwrap())
        .sum::<u16>()
        .to_string()
}
