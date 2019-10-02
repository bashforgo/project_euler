use std::{collections::HashSet, f32};

type Size = u64;

fn len(n: Size) -> Size {
    f32::ceil(f32::log10(n as f32)) as Size
}

fn is_pandigital(mut n: Size) -> bool {
    if len(n) != 9 {
        return false;
    }
    let mut m = 0;
    while n > 0 {
        let r = n % 10;
        n /= 10;
        m |= 1 << r;
    }
    m == 0b11_1111_1110
}

#[cfg(test)]
mod is_pandigital_tests {
    use super::*;

    #[test]
    fn is_pandigital_test() {
        assert_eq!(true, is_pandigital(987_654_321));
        assert_eq!(true, is_pandigital(123_456_789));
        assert_eq!(false, is_pandigital(121_212_121));
        assert_eq!(false, is_pandigital(123_456_788));
        assert_eq!(true, is_pandigital(912_837_465));
        assert_eq!(false, is_pandigital(1_912_837_465));
        assert_eq!(false, is_pandigital(87_654_321));
    }
}

pub fn solve() -> String {
    let mut set = HashSet::new();
    let mut find_pandigital_products = |ll: Size, lh: Size, rl: Size, rh: Size| {
        for l in ll..=lh {
            for r in rl..=rh {
                let prod = l * r;
                let candidate = format!("{}{}{}", l, r, prod).parse::<Size>().unwrap();
                if is_pandigital(candidate) {
                    set.insert(prod);
                    println!("{} x {} = {}", l, r, prod);
                }
            }
        }
    };
    find_pandigital_products(1, 9, 1234, 9876);
    find_pandigital_products(12, 98, 123, 987);

    set.iter().sum::<Size>().to_string()
}
