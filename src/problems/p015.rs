type Size = u128;

fn binomial_coefficient(n: Size, k: Size) -> Size {
    if k > n {
        0
    } else if k == 0 || k == n {
        1
    } else if k > n - k {
        binomial_coefficient(n, n - k)
    } else {
        let l = (1..=k).map(|i| n + 1 - i).product::<Size>();
        let r = (1..=k).product::<Size>();
        l / r
    }
}

#[cfg(test)]
mod binomial_coffeicient_tests {
    use super::*;

    #[test]
    fn works() {
        assert_eq!(binomial_coefficient(4, 2), 6);
        assert_eq!(binomial_coefficient(7, 3), 35);
        assert_eq!(binomial_coefficient(8, 4), 70);
        assert_eq!(binomial_coefficient(40, 20), 137_846_528_820);
    }
}


pub fn solve() -> String {
    let n = 20;
    let k = 20;

    binomial_coefficient(n + k, k).to_string()
}
