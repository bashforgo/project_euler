use std::collections::VecDeque;

type Size = u128;

// trial division
pub fn list(n: Size) -> Vec<Size> {
    let mut n = n;
    let mut factors = vec![];

    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    let mut factor: Size = 3;
    while factor.pow(2) <= n {
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

pub fn componentize(n: Size) -> Vec<(Size, usize)> {
    let mut factors = VecDeque::from(list(n));

    if factors.is_empty() {
        unreachable!("no factors for {}", n)
    } else {
        let mut result = vec![];

        let mut current = factors.pop_front().unwrap();
        let mut count = 1;

        while let Some(f) = factors.pop_front() {
            if f != current {
                result.push((current, count));
                current = f;
                count = 1;
            } else {
                count += 1;
            }
        }
        result.push((current, count));

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_prime_factors() {
        assert_eq!(list(1024), vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2]);
        assert_eq!(list(1890), vec![2, 3, 3, 3, 5, 7]);
        assert_eq!(list(65535), vec![3, 5, 17, 257]);
        assert_eq!(
            list(65536),
            vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]
        );
        assert_eq!(list(161_051), vec![11, 11, 11, 11, 11]);
        assert_eq!(list(123_456_789), vec![3, 3, 3607, 3803]);
    }

    #[test]
    fn lists_components() {
        assert_eq!(componentize(1024), vec![(2, 10)]);
        assert_eq!(componentize(65535), vec![(3, 1), (5, 1), (17, 1), (257, 1)]);
        assert_eq!(componentize(65536), vec![(2, 16)]);
        assert_eq!(
            componentize(277_945_762_500),
            vec![(2, 2), (3, 3), (5, 5), (7, 7)]
        );
    }
}
