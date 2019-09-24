struct Permutation {
    v: Vec<char>,
    c: Vec<usize>,
    i: usize,
    first: bool,
}

impl Permutation {
    fn new(v: Vec<char>) -> Permutation {
        let c = v.iter().map(|_| 0).collect::<Vec<_>>();
        let i = 0;
        let first = true;

        Permutation { v, c, i, first }
    }
}

impl Iterator for Permutation {
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            Some(self.v.clone())
        } else if self.i < self.v.len() {
            if self.c[self.i] < self.i {
                if self.i % 2 == 0 {
                    self.v.swap(0, self.i);
                } else {
                    self.v.swap(self.c[self.i], self.i);
                }

                self.c[self.i] += 1;
                self.i = 0;

                Some(self.v.clone())
            } else {
                self.c[self.i] = 0;
                self.i += 1;
                self.next()
            }
        } else {
            None
        }
    }
}

pub fn solve() -> String {
    let digits = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let p = Permutation::new(digits);
    let mut p = p.map(|p| p.iter().collect::<String>()).collect::<Vec<_>>();
    p.sort();

    p[1_000_000 - 1].to_string()
}
