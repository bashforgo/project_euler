#[inline(always)]
fn divs(n: i32, m: i32) -> bool {
    n % m == 0
}

fn is_prime(n: i32) -> bool {
    if n <= 3 {
        return n > 1;
    } else if divs(n, 2) || divs(n, 3) {
        return false;
    }

    let mut i: i32 = 5;
    while i.pow(2) <= n {
        if divs(n, i) || divs(n, i + 2) {
            return false;
        }

        i += 6;
    }

    return true;
}

pub fn primes() -> Box<dyn Iterator<Item = i32>> {
    Box::new((2..).into_iter().filter(|n| is_prime(*n)))
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
