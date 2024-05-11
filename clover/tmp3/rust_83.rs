
fn starts_or_ends_one(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    18 * 10_i32.pow(n as u32 - 2)
}

fn main() {
    let n = 3; // Example usage
    println!("Count of {}-digit numbers that start or end with 1 is: {}", n, starts_or_ends_one(n));
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
