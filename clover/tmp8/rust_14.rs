
fn all_prefixes(string: String) -> Vec<String> {
    (1..=string.len()).map(|i| string[0..i].to_owned()).collect()
}

fn main() {
    let input = "hello".to_string();
    let prefixes = all_prefixes(input);
    for prefix in prefixes {
        println!("{}", prefix);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_all_prefixes() {
        let v_empty: Vec<String> = vec![];
        assert!(all_prefixes(String::from("")) == v_empty);
        assert!(
            all_prefixes(String::from("asdfgh"))
                == vec!["a", "as", "asd", "asdf", "asdfg", "asdfgh"]
        );
        assert!(all_prefixes(String::from("WWW")) == vec!["W", "WW", "WWW"]);
    }

}
