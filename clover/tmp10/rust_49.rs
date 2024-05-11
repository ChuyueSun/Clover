
fn modp(n: i32, p: i32) -> i32 {
    // Return 2^n modulo p, ensuring that n is non-negative for the exponentiation
    // and mitigating overflow issues by taking the modulus at each step.
    if n < 0 {
        0 // Returning 0 as exponentiation by a negative power isn't supported here.
    } else {
        let mut result = 1;
        for _ in 0..n {
            result = (2 * result) % p;
        }
        result
    }
}

fn main() {
    // Example usage:
    let result = modp(4, 1000);
    println!("2^4 mod 1000 = {}", result); // Output: 2^4 mod 1000 = 16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modp() {
        assert!(modp(3, 5) == 3);
        assert!(modp(1101, 101) == 2);
        assert!(modp(0, 101) == 1);
        assert!(modp(3, 11) == 8);
        assert!(modp(100, 101) == 1);
        assert!(modp(30, 5) == 4);
        assert!(modp(31, 5) == 3);
    }

}
