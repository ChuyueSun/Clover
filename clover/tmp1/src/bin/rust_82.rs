
fn prime_length(s: &str) -> bool {
    let len = s.chars().count();

    if len <= 1 {
        return false;
    }

    for n in 2..=((len as f64).sqrt() as usize) {
        if len % n == 0 {
            return false;
        }
    }

    true
}

fn main() {
    // Example usage
    println!("{}", prime_length("hello")); // Should print true as "hello" length is 5.
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
