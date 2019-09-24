use num::{BigUint, One, Zero};

pub fn solve() -> String {
    let mut f = (BigUint::zero(), BigUint::one());

    let mut n = 0;
    loop {
        n += 1;
        if f.1.to_string().len() >= 1000 {
            break;
        }
        let next = &f.0 + &f.1;
        f = (f.1, next);
    }

    n.to_string()
}
