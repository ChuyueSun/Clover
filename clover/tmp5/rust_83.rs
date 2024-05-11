
fn starts_or_ends_one(n: i32) -> i32 {
    if n <= 0 {
        0
    } else {
        18 * 10_i32.pow(n as u32 - 2) + 2
    }
}

fn main() {
    // Example usage:
    let count = starts_or_ends_one(2);
    println!("The count of numbers with 2 digits starting or ending with 1 is: {}", count);
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
