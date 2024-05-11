
fn starts_one_ends(n: i32) -> i32 {
    if n == 1 {
        // If n is 1, then there is only one number with 1 digit that starts and ends with 1, which is 1 itself.
        return 1;
    }
    // If n is greater than 1, we consider two separate cases: numbers starting with 1 and numbers ending with 1.

    // For numbers starting with 1: We have 1 option for the first digit (1) and 10 options (0-9) for each of the remaining n-1 digits.
    let starts_with_1 = 10_i32.pow((n - 1) as u32);

    // For numbers ending with 1: Again, we have 10 options (1-9 for the first digit and 0-9 for each of the middle digits) for each of the n-1 first digits, and 1 option for the last digit (1).
    let ends_with_1 = 10_i32.pow((n - 1) as u32);

    // We need to subtract the numbers that both start and end with 1, as they are counted twice in the above two calculations.
    let both = 1;

    starts_with_1 + ends_with_1 - both
}

fn main() {
    // Example usage:
    let count = starts_one_ends(2);
    println!("The count of numbers: {}", count);
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
