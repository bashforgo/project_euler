use project_euler::factors;

pub fn solve() -> String {
    let the_number = 600_851_475_143__u128;
    let factors = factors::list(the_number);
    factors.last().unwrap().to_string()
}
