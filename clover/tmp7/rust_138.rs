
fn is_equal_to_sum_even(n: i32) -> bool {
    // Any number greater than or equal to 8 can be represented as the sum of exactly four positive even numbers
    // This is because we can write any even number n >= 8 as: 2 + 2 + 2 + (n - 6). 
    // We exclude numbers less than 8 because the smallest sum of four positive even numbers is 2+2+2+2 = 8
    // Also, the number n must be even itself to be the sum of even numbers.
    n >= 8 && n % 2 == 0
}

fn main() {
    // Example Usage
    println!("Can 12 be written as the sum of exactly 4 positive even numbers? {}", is_equal_to_sum_even(12));
    println!("Can 7 be written as the sum of exactly 4 positive even numbers? {}", is_equal_to_sum_even(7));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_equal_to_sum_even() {
        assert_eq!(is_equal_to_sum_even(4), false);
        assert_eq!(is_equal_to_sum_even(6), false);
        assert_eq!(is_equal_to_sum_even(8), true);
        assert_eq!(is_equal_to_sum_even(10), true);
        assert_eq!(is_equal_to_sum_even(11), false);
        assert_eq!(is_equal_to_sum_even(12), true);
        assert_eq!(is_equal_to_sum_even(13), false);
        assert_eq!(is_equal_to_sum_even(16), true);
    }

}
