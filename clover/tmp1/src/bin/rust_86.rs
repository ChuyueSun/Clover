
fn anti_shuffle(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort_unstable();
            chars.iter().collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    // Example usage:
    let sentence = "hello world";
    let ordered_sentence = anti_shuffle(sentence);
    println!("{}", ordered_sentence); // Should print "ehllo dlorw"
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_anti_shuffle() {
        assert!(anti_shuffle("Hi") == "Hi".to_string());
        assert!(anti_shuffle("hello") == "ehllo".to_string());
        assert!(anti_shuffle("number") == "bemnru".to_string());
        assert!(anti_shuffle("abcd") == "abcd".to_string());
        assert!(anti_shuffle("Hello World!!!") == "Hello !!!Wdlor".to_string());
        assert!(anti_shuffle("") == "".to_string());
        assert!(
            anti_shuffle("Hi. My name is Mister Robot. How are you?")
                == ".Hi My aemn is Meirst .Rboot How aer ?ouy".to_string()
        );
    }

}
