pub fn solve() -> String {
    (2_u32..199_999)
        .filter(|&n| {
            let p = n
                .to_string()
                .drain(..)
                .map(|c| c.to_string().parse::<u32>().unwrap())
                .map(|d| d.pow(5))
                .sum::<u32>();

            n == p
        })
        .sum::<u32>()
        .to_string()
}
