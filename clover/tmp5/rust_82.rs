
fn prime_length(s: &str) -> bool {
    let length = s.len();

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

fn main() {
    // You can test the function with some examples
    println!("{}", prime_length("hello")); // Should print true, as 5 is a prime number
    println!("{}", prime_length("rust")); // Should print false, as 4 is not a prime number
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
