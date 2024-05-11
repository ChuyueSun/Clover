
fn is_equal_to_sum_even(n: i32) -> bool {
    if n < 8 {
        return false; // 4 positive even numbers cannot sum to less than 8.
    }

    // Since every even number can be represented as 2*k for some integer k,
    // we can write our target sum as 2*a + 2*b + 2*c + 2*d, where a, b, c, d
    // are some positive integers.

    // However, if n is even, it can always be written as the sum of 4 positive
    // even numbers, so we check that here.
    n % 2 == 0
}

fn main() {
    let numbers = vec![5, 10, 28, 36, 40, 7];
    for &num in &numbers {
        println!("{} can{} be written as the sum of exactly 4 positive even numbers.", num, if is_equal_to_sum_even(num) { "" } else { "not" });
    }
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
