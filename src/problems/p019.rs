pub fn solve() -> String {
    let mut year = 1901;
    let mut month = 1;
    let mut day_of_week = 1;

    let number_of_days_in_month = |year, month| match month {
        1 => 31,
        2 => {
            if year % 4 == 0 {
                29
            } else {
                28
            }
        }
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => unreachable!(),
    };

    let mut starts_with_sunday = 0;
    while year < 2001 {
        let days_to_add = number_of_days_in_month(year, month);

        if month == 12 {
            year += 1;
            month = 1;
        } else {
            month += 1;
        }

        day_of_week += days_to_add;
        day_of_week %= 7;

        if day_of_week == 6 {
            starts_with_sunday += 1;
        }
    }

    starts_with_sunday.to_string()
}
