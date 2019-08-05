use std::collections::HashMap;

type Size = u16;

fn to_english(n: Size) -> String {
    let exact: HashMap<Size, _> = [
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
        (10, "ten"),
        (11, "eleven"),
        (12, "twelve"),
        (13, "thirteen"),
        (14, "fourteen"),
        (15, "fifteen"),
        (16, "sixteen"),
        (17, "seventeen"),
        (18, "eighteen"),
        (19, "nineteen"),
        (1000, "one thousand"),
    ]
    .iter()
    .cloned()
    .collect();

    let tens: HashMap<Size, _> = [
        (2, "twenty"),
        (3, "thirty"),
        (4, "forty"),
        (5, "fifty"),
        (6, "sixty"),
        (7, "seventy"),
        (8, "eighty"),
        (9, "ninety"),
    ]
    .iter()
    .cloned()
    .collect();

    if let Some(v) = exact.get(&n) {
        String::from(*v)
    } else {
        let s = n.to_string();
        let mut iter = s.chars().map(|c| c.to_string().parse::<Size>().unwrap());

        if s.len() == 2 {
            let ten = iter.next().unwrap();
            let one = iter.next().unwrap();

            if let Some(one) = exact.get(&one) {
                format!("{} {}", tens.get(&ten).unwrap(), one)
            } else {
                String::from(*tens.get(&ten).unwrap())
            }
        } else if s.len() == 3 {
            let hundred = iter.next().unwrap();
            let rest = String::from(&s[1..]).parse::<Size>().unwrap();

            if rest > 0 {
                format!(
                    "{} hundred and {}",
                    exact.get(&hundred).unwrap(),
                    to_english(rest)
                )
            } else {
                format!("{} hundred", exact.get(&hundred).unwrap())
            }

        } else {
            unimplemented!("too big number")
        }
    }
}

#[cfg(test)]
mod to_english_tests {
    use super::*;

    #[test]
    fn exact() {
        assert_eq!(to_english(1), "one");
        assert_eq!(to_english(2), "two");
        assert_eq!(to_english(3), "three");
        assert_eq!(to_english(4), "four");
        assert_eq!(to_english(5), "five");
        assert_eq!(to_english(6), "six");
        assert_eq!(to_english(7), "seven");
        assert_eq!(to_english(8), "eight");
        assert_eq!(to_english(9), "nine");
        assert_eq!(to_english(10), "ten");
        assert_eq!(to_english(11), "eleven");
        assert_eq!(to_english(12), "twelve");
        assert_eq!(to_english(13), "thirteen");
        assert_eq!(to_english(14), "fourteen");
        assert_eq!(to_english(15), "fifteen");
        assert_eq!(to_english(16), "sixteen");
        assert_eq!(to_english(17), "seventeen");
        assert_eq!(to_english(18), "eighteen");
        assert_eq!(to_english(19), "nineteen");
        assert_eq!(to_english(100), "one hundred");
        assert_eq!(to_english(1000), "one thousand");
    }

    #[test]
    fn tens() {
        assert_eq!(to_english(20), "twenty");
        assert_eq!(to_english(30), "thirty");
        assert_eq!(to_english(40), "forty");
        assert_eq!(to_english(50), "fifty");
        assert_eq!(to_english(60), "sixty");
        assert_eq!(to_english(70), "seventy");
        assert_eq!(to_english(80), "eighty");
        assert_eq!(to_english(90), "ninety");

        assert_eq!(to_english(28), "twenty eight");
        assert_eq!(to_english(35), "thirty five");
        assert_eq!(to_english(44), "forty four");
        assert_eq!(to_english(59), "fifty nine");
        assert_eq!(to_english(67), "sixty seven");
        assert_eq!(to_english(76), "seventy six");
        assert_eq!(to_english(83), "eighty three");
        assert_eq!(to_english(92), "ninety two");
    }

    #[test]
    fn hundreds() {
        assert_eq!(to_english(820), "eight hundred and twenty");
        assert_eq!(to_english(530), "five hundred and thirty");
        assert_eq!(to_english(440), "four hundred and forty");
        assert_eq!(to_english(950), "nine hundred and fifty");
        assert_eq!(to_english(760), "seven hundred and sixty");
        assert_eq!(to_english(670), "six hundred and seventy");
        assert_eq!(to_english(380), "three hundred and eighty");
        assert_eq!(to_english(290), "two hundred and ninety");

        assert_eq!(to_english(828), "eight hundred and twenty eight");
        assert_eq!(to_english(535), "five hundred and thirty five");
        assert_eq!(to_english(444), "four hundred and forty four");
        assert_eq!(to_english(959), "nine hundred and fifty nine");
        assert_eq!(to_english(767), "seven hundred and sixty seven");
        assert_eq!(to_english(676), "six hundred and seventy six");
        assert_eq!(to_english(383), "three hundred and eighty three");
        assert_eq!(to_english(292), "two hundred and ninety two");
    }
}


pub fn solve() -> String {
    (1..=1000)
        .map(to_english)
        .collect::<String>()
        .drain(..)
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .len()
        .to_string()
}
