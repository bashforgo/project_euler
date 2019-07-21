fn decompose(n: i64) -> Vec<i64> {
    let mut n = n.clone();
    let mut factors = vec![];

    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    let mut factor = 3;
    while factor ^ 2 <= n {
        if n % factor == 0 {
            factors.push(factor);
            n /= factor;
        } else {
            factor += 2;
        }
    }

    if n != 1 {
        factors.push(n);
    }

    factors
}

pub fn solve() -> String {
    let the_number = 600_851_475_143__i64;
    let factors = decompose(the_number);
    println!("{:?}", factors);
    factors.last().unwrap().to_string()
}
