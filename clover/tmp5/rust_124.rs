
fn valid_date(date: &str) -> bool {
    if date.is_empty() || !date.contains('-') { return false; }
    
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 { return false; }
    
    let month = parts[0].parse::<u32>();
    let day = parts[1].parse::<u32>();
    let year = parts[2].parse::<u32>();
    
    if month.is_err() || day.is_err() || year.is_err() { return false; }
    
    let month = month.unwrap();
    let day = day.unwrap();
    
    if month < 1 || month > 12 { return false; }
    
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => day >= 1 && day <= 31,
        4 | 6 | 9 | 11 => day >= 1 && day <= 30,
        2 => day >= 1 && day <= 29,
        _ => false, // The program will never reach this point as month is always within 1-12
    }
}

fn main() {
    assert_eq!(valid_date("01-15-2021"), true);
    assert_eq!(valid_date("02-30-2021"), false); // Invalid day for February
    assert_eq!(valid_date("13-01-2021"), false); // Invalid month
    assert_eq!(valid_date("00-00-0000"), false); // Invalid date
    assert_eq!(valid_date("12-31-2021"), true);
    assert_eq!(valid_date(""), false); // Empty string
    assert_eq!(valid_date("2-31-2021"), false); // Month without leading zero
    assert_eq!(valid_date("02-29-2021"), true);
    println!("All tests passed!");
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
