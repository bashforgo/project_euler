fn is_palindrome(n: i32) -> bool {
    let s = n.to_string();
    let r: String = s.chars().rev().collect();

    s == r
}

pub fn solve() -> String {
    let min = 900;
    let max = 999;
    let mut res = -1;
    for i in (min..=max).rev() {
        for j in (i..=max).rev() {
            let k = i * j;
            if is_palindrome(k) && k > res {
                res = k;
            }
        }
    }

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palindorme() {
        assert_eq!(is_palindrome(1), true);
        assert_eq!(is_palindrome(12), false);
        assert_eq!(is_palindrome(123), false);
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(1221), true);
        assert_eq!(is_palindrome(1321), false);
        assert_eq!(is_palindrome(13231), true);
        assert_eq!(is_palindrome(9009), true);
        assert_eq!(is_palindrome(901209), false);
    }
}
