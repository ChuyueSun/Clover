
fn starts_one_ends(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    18 * 10_i32.pow((n - 2) as u32)
}

fn main() {
    println!("Count with 1 digit: {}", starts_or_ends_with_one(1));
    println!("Count with 2 digits: {}", starts_or_ends_with_one(2));
    println!("Count with 3 digits: {}", starts_or_ends_with_one(3));
    // You can test with more digits...
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starts_one_ends() {
        assert!(starts_one_ends(1) == 1);
        assert!(starts_one_ends(2) == 18);
        assert!(starts_one_ends(3) == 180);
        assert!(starts_one_ends(4) == 1800);
        assert!(starts_one_ends(5) == 18000);
    }

}
