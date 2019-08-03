pub struct Fib(i32, i32);

impl Default for Fib {
    fn default() -> Fib {
        Fib(0, 1)
    }
}

impl Iterator for Fib {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        let tmp = self.0 + self.1;
        self.0 = self.1;
        self.1 = tmp;

        Some(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_few() {
        let first_few: Vec<_> = Fib::default().take(10).collect();
        assert_eq!(first_few, vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55]);
    }
}
