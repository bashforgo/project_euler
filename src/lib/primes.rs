type Size = u128;

pub fn is_prime(n: Size) -> bool {
    if n <= 3 {
        return n > 1;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i: Size = 5;
    while i.pow(2) <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }

        i += 6;
    }

    true
}

pub fn primes() -> Box<dyn Iterator<Item = Size>> {
    Box::new((2..).filter(|n| is_prime(*n)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_few_primes() {
        let first_20: Vec<_> = primes().take(20).collect();
        assert_eq!(
            first_20,
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71]
        );
    }
}
