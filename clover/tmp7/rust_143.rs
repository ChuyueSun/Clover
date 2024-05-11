
/// Given a string representing a sentence, returns a string that contains
/// the words from the original sentence whose lengths are prime numbers.
/// 
/// # Arguments
/// 
/// * `sentence` - A string slice that holds the sentence.
/// 
/// # Returns
/// 
/// A `String` that contains the filtered words separated by spaces.
fn words_in_sentence(sentence: &str) -> String {
    fn is_prime(n: usize) -> bool {
        if n <= 1 {
            return false;
        }
        for i in 2..=((n as f64).sqrt() as usize) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    sentence
        .split_whitespace()
        .filter(|word| is_prime(word.len()))
        .collect::<Vec<&str>>()
        .join(" ")
}

fn main() {
    let sentence = "Rust is an empowering language to develop in";
    let prime_length_words = words_in_sentence(sentence);
    println!("{}", prime_length_words);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_words_in_sentence() {
        assert_eq!(words_in_sentence("This is a test"), "is");
        assert_eq!(words_in_sentence("lets go for swimming"), "go for");
        assert_eq!(
            words_in_sentence("there is no place available here"),
            "there is no place"
        );
        assert_eq!(words_in_sentence("Hi I am Hussein"), "Hi am Hussein");
        assert_eq!(words_in_sentence("go for it"), "go for it");
        assert_eq!(words_in_sentence("here"), "");
        assert_eq!(words_in_sentence("here is"), "is");
    }

}
