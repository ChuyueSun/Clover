
fn modp(n: i32, p: i32) -> i32 {
    // Return 2^n modulo p using exponentiation by squaring to handle large exponents.
    let mut result = 1;
    let mut base = 2;
    let mut exponent = n;
    let modulus = p;

    // Iteratively compute result = (base^exponent) % modulus.
    while exponent > 0 {
        // If the current exponent is odd, multiply the result by the base (mod modulus)
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        // Square the base (mod modulus)
        base = (base * base) % modulus;
        // Halve the exponent
        exponent /= 2;
    }
    result
}

fn main() {
    let n = 10;
    let p = 1000;
    println!("2^{} modulo {} is {}", n, p, modp(n, p));
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
