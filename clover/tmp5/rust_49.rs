
fn modp(n: i32, p: i32) -> i32 {
    let mut result = 1;
    let base = 2;

    // Convert n and p to positive to avoid negative number handling.
    let mut exponent = n;
    let modulus = p.abs();

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent >>= 1;
        base = (base * base) % modulus;
    }

    result % modulus
}

fn main() {
    let n = 4; 
    let p = 7;

    // Example call to the `modp` function.
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
