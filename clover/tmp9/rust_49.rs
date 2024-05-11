
fn modp(n: i32, p: i32) -> i32 {
    let mut result = 1;
    let base = 2;

    // Convert n to unsigned to deal with negative powers
    let mut exp = n as u32;

    // Modulo p to avoid integer overflow for very large exponents
    let mut base_mod = base % p;

    while exp > 0 {
        if exp % 2 == 1 {
            result *= base_mod;
            result %= p;
        }
        base_mod = (base_mod * base_mod) % p;
        exp /= 2;
    }

    result
}

fn main() {
    // Example usage:
    let n = 10; // For 2^10
    let p = 1000;
    println!("2^{} modulo {} is: {}", n, p, modp(n, p));
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
