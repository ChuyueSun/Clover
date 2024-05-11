
fn modp(n: i32, p: i32) -> i32 {
    // Calculate 2^n modulo p
    let mut exponent = n;
    let mut base: i64 = 2; // use i64 to prevent overflow
    let mut result: i64 = 1;
    let modulus = p as i64;

    // Use the binary exponentiation algorithm
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent /= 2;
        base = (base * base) % modulus;
    }

    // Cast the result back to i32, ensuring it is within the bounds of the type
    (result % modulus) as i32
}

fn main() {
    // Example usage:
    let n = 10;
    let p = 1000;
    println!("2^{} mod {} = {}", n, p, modp(n, p));
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
