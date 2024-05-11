
fn prime_length(s: &str) -> bool {
    let length = s.chars().count();
    if length <= 1 {
        return false;
    }
    
    for i in 2..=(length as f64).sqrt() as usize {
        if length % i == 0 {
            return false;
        }
    }
    
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(prime_length(""), false);
    }

    #[test]
    fn test_single_character() {
        assert_eq!(prime_length("a"), false);
    }

    #[test]
    fn test_prime_length() {
        assert_eq!(prime_length("hello"), true); // 5 characters
    }

    #[test]
    fn test_non_prime_length() {
        assert_eq!(prime_length("hello!"), false); // 6 characters
    }
}

fn main() {
    // Example usage:
    println!("Is 'rust' of prime length? {}", prime_length("rust")); // Should output true
    println!("Is 'hello' of prime length? {}", prime_length("hello")); // Should output true
    println!("Is 'world' of prime length? {}", prime_length("world")); // Should output false
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
