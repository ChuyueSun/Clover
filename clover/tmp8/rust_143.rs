
fn words_in_sentence(sentence: &str) -> String {
    fn is_prime(num: usize) -> bool {
        if num <= 1 {
            false
        } else if num <= 3 {
            true
        } else if num % 2 == 0 || num % 3 == 0 {
            false
        } else {
            let mut i: usize = 5;
            while i * i <= num {
                if num % i == 0 || num % (i + 2) == 0 {
                    return false;
                }
                i += 6;
            }
            true
        }
    }

    sentence
        .split_whitespace()
        .filter(|word| is_prime(word.len()))
        .collect::<Vec<&str>>()
        .join(" ")
}

fn main() {
    let sentence = "This is a simple example of a sentence with mixed lengths";
    let result = words_in_sentence(sentence);
    println!("{}", result);
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
