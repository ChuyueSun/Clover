
fn even_odd_palindrome(n: i32) -> (i32, i32) {
    let mut even_count = 0;
    let mut odd_count = 0;

    for i in 1..=n {
        if is_palindrome(i) {
            match i % 2 {
                0 => even_count += 1,
                _ => odd_count += 1,
            }
        }
    }

    (even_count, odd_count)
}

fn is_palindrome(x: i32) -> bool {
    let x_str = x.to_string();
    x_str.chars().zip(x_str.chars().rev()).all(|(l, r)| l == r)
}

fn main() {
    // Example usage:
    let (even_palindromes, odd_palindromes) = even_odd_palindrome(1000);
    println!("Even palindromes: {}, Odd palindromes: {}", even_palindromes, odd_palindromes);
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