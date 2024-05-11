
fn valid_date(date: &str) -> bool {
    if date.is_empty() {
        return false;
    }

    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        // Date format must be mm-dd-yyyy
        return false;
    }

    if let (Ok(month), Ok(day), Ok(year)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>(), parts[2].parse::<u32>()) {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => day >= 1 && day <= 31,
            4 | 6 | 9 | 11 => day >= 1 && day <= 30,
            2 if is_leap_year(year) => day >= 1 && day <= 29,
            2 => day >= 1 && day <= 28,
            _ => false, // Month is not in the range 1-12
        }
    } else {
        false // One of the parts is not a valid number
    }
}

fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn main() {
    // Test cases
    println!("{}", valid_date("01-15-2020")); // true
    println!("{}", valid_date("02-29-2020")); // true, leap year
    println!("{}", valid_date("02-30-2021")); // false
    println!("{}", valid_date("13-15-2020")); // false, invalid month
    println!("{}", valid_date("00-00-0000")); // false, invalid date
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
