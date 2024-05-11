
fn even_odd_palindrome(n: i32) -> (i32, i32) {
    fn is_palindrome(num: i32) -> bool {
        let num_str = num.to_string();
        num_str == num_str.chars().rev().collect::<String>()
    }

    let (mut evens, mut odds) = (0, 0);

    for i in 1..=n {
        if is_palindrome(i) {
            if i % 2 == 0 {
                evens += 1;
            } else {
                odds += 1;
            }
        }
    }
    
    (evens, odds)
}

fn main() {
    // Example usage:
    println!("{:?}", even_odd_palindrome(100)); // Outputs: (5, 9)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_odd_palindrome() {
        assert!(even_odd_palindrome(123) == (8, 13));
        assert!(even_odd_palindrome(12) == (4, 6));
        assert!(even_odd_palindrome(3) == (1, 2));
        assert!(even_odd_palindrome(63) == (6, 8));
        assert!(even_odd_palindrome(25) == (5, 6));
        assert!(even_odd_palindrome(19) == (4, 6));
        assert!(even_odd_palindrome(9) == (4, 5));
        assert!(even_odd_palindrome(1) == (0, 1));
    }

}
