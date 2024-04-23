fn main() {}

/*

    Given a positive integer n, return a tuple that has the number of even and odd
    integer palindromes that fall within the range(1, n), inclusive.

    Note:
        1. 1 <= n <= 10^3
        2. returned tuple has the number of even and odd integer palindromes respectively.

*/

fn even_odd_palindrome(n: i32) -> (i32, i32) {
    let mut even = 0;
    let mut odd = 0;

    for i in 1..n + 1 {
        let w: String = i.to_string();
        let p: String = w.chars().rev().collect();

        if w == p && i % 2 == 1 {
            odd += 1;
        }
        if w == p && i % 2 == 0 {
            even += 1;
        }
    }
    (even, odd)
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
