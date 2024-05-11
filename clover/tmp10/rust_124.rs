
fn valid_date(date: &str) -> bool {
    fn is_leap_year(year: u32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
    }

    // Validate the date string format
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        return false;
    }
    
    let (mm, dd, yyyy) = (parts[0], parts[1], parts[2]);
    let month: i32 = match mm.parse() {
        Ok(num) => num,
        Err(_) => return false,
    };
    let day: i32 = match dd.parse() {
        Ok(num) => num,
        Err(_) => return false,
    };
    let year: u32 = match yyyy.parse() {
        Ok(num) => num,
        Err(_) => return false,
    };

    // Validate the month
    if month < 1 || month > 12 {
        return false;
    }

    // Validate the day
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => day >= 1 && day <= 31,
        4 | 6 | 9 | 11 => day >= 1 && day <= 30,
        2 => if is_leap_year(year) { day >= 1 && day <= 29 } else { day >= 1 && day <= 28 },
        _ => false, // We have already checked for invalid months above, but added for completeness
    }
}

fn main() {
    // Tests
    assert_eq!(valid_date("01-15-2021"), true);
    assert_eq!(valid_date("02-29-2020"), true);
    assert_eq!(valid_date("02-30-2020"), false);
    assert_eq!(valid_date("13-01-2021"), false);
    assert_eq!(valid_date("01-01-21"), false);
    assert_eq!(valid_date(""), false);
    assert_eq!(valid_date("12-31-1999"), true);
    assert_eq!(valid_date("06-31-2021"), false);
    println!("Done testing");
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
