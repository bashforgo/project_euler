use num::{cast::FromPrimitive, pow::Pow, BigUint};

pub fn solve() -> String {
    let p = BigUint::from_u8(2).unwrap().pow(1000_u16);
    p.to_string()
        .drain(..)
        .map(|c| c.to_string().parse::<u16>().unwrap())
        .sum::<u16>()
        .to_string()
}
