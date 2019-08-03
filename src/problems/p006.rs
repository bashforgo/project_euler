pub fn solve() -> String {
    let sum_of_squares = (1..=100_i32).map(|n| n.pow(2)).sum::<i32>();
    let square_of_sum = (1..=100_i32).sum::<i32>().pow(2);

    (square_of_sum - sum_of_squares).to_string()
}