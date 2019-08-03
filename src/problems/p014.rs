use project_euler::collatz::Collatz;
use std::collections::HashMap;

pub fn solve() -> String {
    let mut lengths = HashMap::new();

    for starting in 1..=1_000_000 {
        let seq = Collatz::new(starting);
        let mut len = 0;

        for c in seq {
            if let Some(length) = lengths.get(&c) {
                len += length;
                break;
            }
            len += 1;
        }

        lengths.insert(starting, len);
    }

    lengths.iter().max_by_key(|kv| kv.1).unwrap().0.to_string()
}
