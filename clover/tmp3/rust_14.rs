
fn all_prefixes(string: &str) -> Vec<String> {
    (1..=string.len()).map(|i| string[..i].to_string()).collect()
}

fn main() {
    let input_string = "rust";
    let prefixes = all_prefixes(input_string);

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
