
fn starts_or_ends_one(n: i32) -> i32 {
    if n < 1 {
        0
    } else if n == 1 {
        9
    } else {
        // There are two cases for n-digit numbers: those starting with 1 and
        // those ending with 1, but we should subtract the count of numbers that
        // both start and end with 1 to avoid double counting.
        let starts_with_one = 10_i32.pow(n as u32 - 1);
        let ends_with_one = starts_with_one;
        let both_start_and_end_with_one = 10_i32.pow(n as u32 - 2);
        
        starts_with_one + ends_with_one - both_start_and_end_with_one
    }
}

fn main() {
    // Example usage:
    let number_of_digits = 3;
    println!("Count of {}-digit numbers that start or end with 1: {}", number_of_digits, starts_or_ends_one(number_of_digits));
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
