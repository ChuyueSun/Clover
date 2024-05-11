
fn valid_date(date: &str) -> bool {
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 { return false; }
    
    // Parse month, day, year
    let month: u32 = match parts[0].parse() {
        Ok(m) => m,
        Err(_) => return false,
    };
    let day: u32 = match parts[1].parse() {
        Ok(d) => d,
        Err(_) => return false,
    };
    let year: u32 = match parts[2].parse() {
        Ok(y) => y,
        Err(_) => return false,
    };
    
    if year < 1 || month < 1 || month > 12 || day < 1 { return false; }

    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => day <= 31,
        4 | 6 | 9 | 11 => day <= 30,
        2 => day <= 29,
        _ => false,
    }
}

fn main() {
    // You can test the function with some example dates:
    println!("{}", valid_date("12-31-2020")); // Should return true
    println!("{}", valid_date("02-29-2021")); // Should return false
    println!("{}", valid_date("01-01-2020")); // Should return true
    println!("{}", valid_date("11-31-2020")); // Should return false
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
