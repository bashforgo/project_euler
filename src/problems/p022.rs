const NAMES: &str = include_str!("assets/p022.txt");

const A: u8 = 65;

pub fn solve() -> String {
    let names = NAMES.to_string();
    let names = names.replace('"', "");
    let mut names = names.split(',').collect::<Vec<_>>();
    names.sort();

    names
        .iter()
        .zip(1..)
        .map(|(&n, i)| {
            let score = u32::from(n.as_bytes().iter().map(|c| c - A + 1).sum::<u8>());
            i * score
        })
        .sum::<u32>()
        .to_string()
}
