use project_euler::integer;

type Size = u128;

fn lcm(a: Size, b: Size) -> Size {
    (a * b) / integer::gcd(a, b)
}

pub fn solve() -> String {
    (2..20).fold(1, lcm).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _lcm() {
        assert_eq!(lcm(21, 6), 42);
    }
}
