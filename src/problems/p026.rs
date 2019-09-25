use std::collections::HashMap;

pub fn find_cycle(num: u32, den: u32) -> Option<String> {
    let mut res = String::new();
    let mut map = HashMap::new();
    let mut rem = num % den;
    loop {
        if rem == 0 {
            return None;
        }
        if map.contains_key(&rem) {
            return Some(res[*map.get(&rem).unwrap()..].to_string());
        }

        map.insert(rem, res.len());

        rem *= 10;
        let partial = rem / den;
        res += &partial.to_string();

        rem %= den;
    }
}

pub fn solve() -> String {
    (2..1000)
        .map(|n| (n, find_cycle(1, n).unwrap_or_else(|| "".into()).len()))
        .max_by_key(|t| t.1)
        .unwrap()
        .0
        .to_string()
}
