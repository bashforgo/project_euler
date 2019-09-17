use project_euler::factors;

type Size = u128;

pub fn solve() -> String {
    ((2 as Size)..)
        .map(|i| (i * (i + 1)) / 2)
        .map(|t| {
            let c = factors::componentize(t);
            let n = c.iter().map(|(_, a)| (a + 1) as Size).product::<Size>();
            (t, n)
        })
        .filter(|&(_, n)| n >= 500)
        .take(1)
        .last()
        .unwrap()
        .0
        .to_string()
}
