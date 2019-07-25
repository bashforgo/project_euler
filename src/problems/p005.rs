fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }

    a
}

fn lcm(a: i32, b: i32) -> i32 {
    (a * b) / gcd(a, b)
}

pub fn solve() -> String {
    (2..20)
        .into_iter()
        .fold(1, |acc, cur| lcm(acc, cur))
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _gcd() {
        assert_eq!(gcd(1071, 462), 21);
    }

    #[test]
    fn _lcm() {
        assert_eq!(lcm(21, 6), 42);
    }
}
