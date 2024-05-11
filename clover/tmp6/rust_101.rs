
fn words_string(s: &str) -> Vec<String> {
    s.split(',')
        .flat_map(|part| part.split_whitespace())
        .filter(|word| !word.is_empty())
        .map(str::to_string)
        .collect()
}

fn main() {
    let test_string = "hello,world, this,is, a test";
    let words = words_string(test_string);
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
