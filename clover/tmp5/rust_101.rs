
fn words_string(s: &str) -> Vec<String> {
    s.split(|c: char| c.is_whitespace() || c == ',')
        .filter(|word| !word.is_empty())
        .map(String::from)
        .collect()
}

fn main() {
    let s = "Hello, world, this is, a test";
    let words = words_string(s);
    for word in &words {
        println!("{}", word);
    }
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
