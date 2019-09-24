pub fn solve() -> String {
    let mut digits = *b"0123456789";

    let mut n = 0;
    let mut res = None;
    loop {
        n += 1;
        if n == 1_000_000 {
            res = Some(String::from_utf8_lossy(&digits).to_string());
            break;
        }

        let mut first_char = None;
        for i in (1..digits.len()).rev() {
            if digits[i - 1] < digits[i] {
                first_char = Some(i - 1);
                break;
            }
        }

        if first_char.is_none() {
            break;
        }

        let first_char = first_char.unwrap();

        let mut ceil = first_char + 1;
        for i in first_char + 1..digits.len() {
            if digits[i] > digits[first_char] && digits[i] < digits[ceil] {
                ceil = i;
            }
        }

        digits.swap(first_char, ceil);

        let right_of_first = &mut digits[first_char + 1..];
        right_of_first.reverse();
    }

    res.unwrap()
}
