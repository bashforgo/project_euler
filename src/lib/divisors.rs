use super::factors;

type Size = u128;

pub fn sum_of_proper_divisors(n: Size) -> Size {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_proper_divisors_() {
        assert_eq!(sum_of_proper_divisors(12), 16);
        assert_eq!(sum_of_proper_divisors(28), 28);
        assert_eq!(sum_of_proper_divisors(220), 284);
        assert_eq!(sum_of_proper_divisors(284), 220);
    }
}
