
fn valid_date(date: &str) -> bool {
    if date.len() != 10 {
        return false;
    }

    let mut parts = date.split('-');
    let month_part = parts.next();
    let day_part = parts.next();
    let year_part = parts.next();

    if month_part.is_none() || day_part.is_none() || year_part.is_none() {
        return false;
    }

    let month = month_part.unwrap().parse::<u32>();
    let day = day_part.unwrap().parse::<u32>();
    let year = year_part.unwrap().parse::<u32>();

    if month.is_err() || day.is_err() || year.is_err() {
        return false;
    }

    let month = month.unwrap();
    let day = day.unwrap();
    let _year = year.unwrap(); // year is not actually used for validation in these rules

    if month < 1 || month > 12 {
        return false;
    }

    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => day >= 1 && day <= 31,
        4 | 6 | 9 | 11 => day >= 1 && day <= 30,
        2 => day >= 1 && day <= 29,
        _ => false,
    }
}

fn main() {
    // You can test the function with some examples
    assert_eq!(valid_date("01-31-2020"), true);
    assert_eq!(valid_date("02-29-2020"), true);
    assert_eq!(valid_date("02-30-2020"), false);
    assert_eq!(valid_date("04-31-2020"), false);
    assert_eq!(valid_date("13-01-2020"), false);
    assert_eq!(valid_date(""), false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_date() {
        assert_eq!(valid_date("03-11-2000"), true);
        assert_eq!(valid_date("15-01-2012"), false);
        assert_eq!(valid_date("04-0-2040"), false);
        assert_eq!(valid_date("06-04-2020"), true);
        assert_eq!(valid_date("01-01-2007"), true);
        assert_eq!(valid_date("03-32-2011"), false);
        assert_eq!(valid_date(""), false);
        assert_eq!(valid_date("04-31-3000"), false);
        assert_eq!(valid_date("06-06-2005"), true);
        assert_eq!(valid_date("21-31-2000"), false);
        assert_eq!(valid_date("04-12-2003"), true);
        assert_eq!(valid_date("04122003"), false);
        assert_eq!(valid_date("20030412"), false);
        assert_eq!(valid_date("2003-04"), false);
        assert_eq!(valid_date("2003-04-12"), false);
        assert_eq!(valid_date("04-2003"), false);
    }

}
