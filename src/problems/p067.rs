use super::p018::maximum_path_sum;

const TRIANGLE: &str = include_str!("assets/p067.txt");

pub fn solve() -> String {
    maximum_path_sum(TRIANGLE).to_string()
}
