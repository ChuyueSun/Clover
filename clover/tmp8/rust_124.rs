 the Rust program that implements the functionality described in the docstring:

```rust
fn valid_date(date: &str) -> bool {
    // Check if the date string is empty
    if date.is_empty() {
        return false;
    }

    // Check the date format: mm-dd-yyyy
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        return false;
    }

    let month = parts[0].parse::<u32>();
    let day = parts[1].parse::<u32>();
    let year = parts[2].parse::<u32>();

    // Validate parsed parts
    match (month, day, year) {
        (Ok(m), Ok(d), Ok(_)) if m >= 1 && m <= 12 => {
            match m {
                // Months with 31 days
                1 | 3 | 5 | 7 | 8 | 10 | 12 => d >= 1 && d <= 31,
                // Months with 30 days
                4 | 6 | 9 | 11 => d >= 1 && d <= 30,
                // February, with leap year consideration not included
                2 => d >= 1 && d <= 29,
                // Any other month number is invalid
                _ => false,
            }
        },
        _ => false,
    }
}

fn main() {
    // Example usage
    let date = "02-29-2023";
    println!("The date {} is valid: {}", date, valid_date(date));
}
```

Please note that this program does not check for leap years, which is a factor in assessing the validity of the date particularly for the 29th of February. If leap year consideration is required, additional logic would need to be implement
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
