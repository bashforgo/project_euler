type Size = u32;

fn digit_factorial(d: Size) -> Size {
    match d {
        0 => 1,
        1 => 1,
        2 => 2,
        3 => 6,
        4 => 24,
        5 => 120,
        6 => 720,
        7 => 5040,
        8 => 40320,
        9 => 362_880,
        _ => unreachable!("{} is not a digit", d),
    }
}

pub fn solve() -> String {
    let mut sum = 0;
    for n in 10..999_999 {
        let mut m = n;
        let mut digits = vec![];
        while m > 0 {
            let d = m % 10;
            digits.push(d);
            m /= 10;
        }
        let f = digits.into_iter().map(digit_factorial).sum::<Size>();
        if n == f {
            sum += n;
        }
    }
    sum.to_string()
}
