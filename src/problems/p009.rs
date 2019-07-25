pub fn solve() -> String {
    let mut triplets = vec![];

    for a in 1_i32..1000 {
        for b in a + 1..1000 {
            let c = 1000 - a - b;
            if b < c {
                triplets.push((a, b, 1000 - a - b));
            }
        }
    }

    let triplet = triplets
        .into_iter()
        .filter(|(a, b, c)| a.pow(2) + b.pow(2) == c.pow(2))
        .take(1)
        .collect::<Vec<_>>();
    if let [(a, b, c)] = &triplet[..] {
        (a * b * c).to_string()
    } else {
        unreachable!()
    }
}