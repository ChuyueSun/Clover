
fn all_prefixes(string: String) -> Vec<String> {
    let mut prefixes = Vec::new();
    for end in 1..=string.len() {
        prefixes.push(string[..end].to_string());
    }
    prefixes
}

fn main() {
    let input = "Rust".to_string();
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
