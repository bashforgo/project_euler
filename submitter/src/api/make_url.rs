#[macro_export]
macro_rules! make_url {
    ($($arg:tt)*) => (&format!("{}{}", "https://projecteuler.net", format!($($arg)*)));
}
