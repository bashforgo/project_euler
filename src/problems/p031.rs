fn coin_combinations(left: i32, coins: &[i32]) -> i32 {
    if coins.is_empty() || left < 0 {
        0
    } else if left == 0 {
        1
    } else {
        let (head, tail) = coins.split_first().unwrap();
        coin_combinations(left - head, coins) + coin_combinations(left, tail)
    }
}

pub fn solve() -> String {
    coin_combinations(200, &[1, 2, 5, 10, 20, 50, 100, 200]).to_string()
}
