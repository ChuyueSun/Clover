
fn starts_or_ends_with_one(n: i32) -> i32 {
    if n < 1 {
        return 0; // Ensure n is at least 1
    }
    if n == 1 {
        return 1; // There's only one one-digit number that starts or ends with 1, which is 1 itself.
    }

    // For n > 1, consider the combinations where:
    //   - The starting digit is 1 (other digits can be anything)
    //   - The ending digit is 1 and the first digit is not (other digits can be anything)
    let starts_with_one = 9_i32.pow((n - 1) as u32);
    let ends_with_one = 9_i32.pow((n - 1) as u32);
    
    // Subtract the count where both the start and end are 1 as this is counted twice
    let both_ends_one = 1_i32.pow((n - 2) as u32); // Always 1 for n>=2 

    starts_with_one + ends_with_one - both_ends_one 
}

fn main() {
    let n = 3; // Example usage
    println!("Count of {}-digit numbers that start or end with 1: {}", n, starts_or_ends_with_one(n));
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
