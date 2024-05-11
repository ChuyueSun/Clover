
fn modp(n: i32, p: i32) -> i32 {
    let mut result = 1;
    let mut base = 2;

    let mut exponent = n;
    let modulus = p;

    // Exponentiation by squaring algorithm to handle large exponents 
    // and prevent integer overflows.
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;  // Multiply base to result for current bit and take mod
        }
        base = (base * base) % modulus;  // Square the base and take mod
        exponent /= 2;  // Move to the next bit
    }

    result
}

fn main() {
    // Example usage:
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
