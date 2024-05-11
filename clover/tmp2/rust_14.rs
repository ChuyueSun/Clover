
fn all_prefixes(string: String) -> Vec<String> {
    let mut prefixes = Vec::new();
    for i in 0..string.len() {
        prefixes.push(string[..=i].to_string());
    }
    prefixes
}

fn main() {
    let input_string = String::from("rust");
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
