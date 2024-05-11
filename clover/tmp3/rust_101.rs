
fn words_string(s: &str) -> Vec<&str> {
    s.split(|c: char| c.is_whitespace() || c == ',')
        .filter(|word| !word.is_empty())
        .collect()
}

fn main() {
    // Example usage:
    let input = "This is a test,string,separated by spaces,and commas";
    let words = words_string(input);
    println!("{:?}", words);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_words_string() {
        assert!(words_string("Hi, my name is John") == vec!["Hi", "my", "name", "is", "John"]);
        assert!(
            words_string("One, two, three, four, five, six")
                == vec!["One", "two", "three", "four", "five", "six"]
        );
        assert!(words_string("Hi, my name") == vec!["Hi", "my", "name"]);
        assert!(
            words_string("One,, two, three, four, five, six,")
                == vec!["One", "two", "three", "four", "five", "six"]
        );
        let v_empty: Vec<String> = vec![];
        assert!(words_string("") == v_empty);
        assert!(words_string("ahmed , gamal") == vec!["ahmed", "gamal"]);
    }

}
