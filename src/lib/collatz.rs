type Size = u128;

pub struct Collatz(Size);

impl Collatz {
    pub fn new(n: u128) -> Collatz {
        Collatz(n)
    }
}

impl Iterator for Collatz {
    type Item = Size;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let tmp = self.0;

            if self.0 == 1 {
                self.0 = 0;
            } else if self.0 % 2 == 0 {
                self.0 /= 2;
            } else {
                self.0 = self.0 * 3 + 1;
            }

            Some(tmp)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let seq = Collatz::new(13).collect::<Vec<_>>();
        assert_eq!(seq, vec![13, 40, 20, 10, 5, 16, 8, 4, 2, 1]);
        let seq = Collatz::new(27).collect::<Vec<_>>();
        assert_eq!(
            seq,
            vec![
                27, 82, 41, 124, 62, 31, 94, 47, 142, 71, 214, 107, 322, 161, 484, 242, 121, 364,
                182, 91, 274, 137, 412, 206, 103, 310, 155, 466, 233, 700, 350, 175, 526, 263, 790,
                395, 1186, 593, 1780, 890, 445, 1336, 668, 334, 167, 502, 251, 754, 377, 1132, 566,
                283, 850, 425, 1276, 638, 319, 958, 479, 1438, 719, 2158, 1079, 3238, 1619, 4858,
                2429, 7288, 3644, 1822, 911, 2734, 1367, 4102, 2051, 6154, 3077, 9232, 4616, 2308,
                1154, 577, 1732, 866, 433, 1300, 650, 325, 976, 488, 244, 122, 61, 184, 92, 46, 23,
                70, 35, 106, 53, 160, 80, 40, 20, 10, 5, 16, 8, 4, 2, 1
            ]
        );
    }
}
