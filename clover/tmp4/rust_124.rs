
fn valid_date(date: &str) -> bool {
    if date.len() != 10 || !date.chars().nth(2).unwrap().eq(&'-') || !date.chars().nth(5).unwrap().eq(&'-') {
        return false;
    }

    // Split the string into components.
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        return false;
    }

    // Parse month, day, and year.
    let month: u32 = match parts[0].parse() {
        Ok(num) => num,
        Err(_) => return false,
    };
    
    let day: u32 = match parts[1].parse() {
        Ok(num) => num,
        Err(_) => return false,
    };
    
    let year: u32 = match parts[2].parse() {
        Ok(num) => num,
        Err(_) => return false,
    };
    
    // Check the month range.
    if month < 1 || month > 12 {
        return false;
    }

    // Check the day range.
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => day >= 1 && day <= 31,
        4 | 6 | 9 | 11 => day >= 1 && day <= 30,
        2 => {
            if is_leap_year(year) {
                day >= 1 && day <= 29
            } else {
                day >= 1 && day <= 28
            }
        },
        _ => false,
    }
}

// Helper function to determine if a year is a leap year.
fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

// Main function for test purpose
fn main() {
    println!("{}", valid_date("01-15-2023")); // Prints true
    println!("{}", valid_date("02-29-2023")); // Prints false
    println!("{}", valid_date("02-29-2024")); // Prints true
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
