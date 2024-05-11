
fn prime_length(s: &str) -> bool {
    let length = s.chars().count();
    // 0 and 1 are not prime numbers
    if length <= 1 {
        return false;
    }
    // 2 is the first prime number
    if length == 2 {
        return true;
    }
    // Even numbers greater than 2 are not prime
    if length % 2 == 0 {
        return false;
    }
    // Check for factors up to the square root of the length
    let limit = (length as f64).sqrt() as usize;
    for i in (3..=limit).step_by(2) {
        if length % i == 0 {
            return false;
        }
    }
    true
}

// Tests
fn main() {
    println!("{}", prime_length("abc")); // true: length 3 is prime
    println!("{}", prime_length("abcdefgh")); // false: length 8 is not prime
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_length() {
        assert!(prime_length("Hello") == true);
        assert!(prime_length("abcdcba") == true);
        assert!(prime_length("kittens") == true);
        assert!(prime_length("orange") == false);
        assert!(prime_length("wow") == true);
        assert!(prime_length("world") == true);
        assert!(prime_length("MadaM") == true);
        assert!(prime_length("Wow") == true);
        assert!(prime_length("") == false);
        assert!(prime_length("HI") == true);
        assert!(prime_length("go") == true);
        assert!(prime_length("gogo") == false);
        assert!(prime_length("aaaaaaaaaaaaaaa") == false);
        assert!(prime_length("Madam") == true);
        assert!(prime_length("M") == false);
        assert!(prime_length("0") == false);
    }

}
