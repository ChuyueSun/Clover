
use std::collections::HashMap;

fn check_dict_case(dict: HashMap<&str, &str>) -> bool {
    if dict.is_empty() {
        return false;
    }

    let all_lower = dict.keys().all(|key| key.chars().all(char::is_lowercase));
    let all_upper = dict.keys().all(|key| key.chars().all(char::is_uppercase));

    all_lower || all_upper
}

fn main() {
    // Example usage:
    let dict_upper = HashMap::from([
        ("ONE", "Value 1"),
        ("TWO", "Value 2"),
    ]);

    let dict_lower = HashMap::from([
        ("one", "Value 1"),
        ("two", "Value 2"),
    ]);

    let dict_mixed = HashMap::from([
        ("One", "Value 1"),
        ("TWO", "Value 2"),
    ]);

    let dict_empty: HashMap<&str, &str> = HashMap::new();

    println!("{}", check_dict_case(dict_upper)); // Should print true
    println!("{}", check_dict_case(dict_lower)); // Should print true
    println!("{}", check_dict_case(dict_mixed)); // Should print false
    println!("{}", check_dict_case(dict_empty)); // Should print false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_dict_case() {
        assert!(check_dict_case(HashMap::from([("p", "pineapple"), ("b", "banana")])) == true);
        assert!(
            check_dict_case(HashMap::from([
                ("p", "pineapple"),
                ("A", "banana"),
                ("B", "banana")
            ])) == false
        );
        assert!(
            check_dict_case(HashMap::from([
                ("p", "pineapple"),
                ("5", "banana"),
                ("a", "apple")
            ])) == false
        );
        assert!(
            check_dict_case(HashMap::from([
                ("Name", "John"),
                ("Age", "36"),
                ("City", "Houston")
            ])) == false
        );
        assert!(check_dict_case(HashMap::from([("STATE", "NC"), ("ZIP", "12345")])) == true);
        assert!(check_dict_case(HashMap::from([("fruit", "Orange"), ("taste", "Sweet")])) == true);
        assert!(check_dict_case(HashMap::new()) == false);
    }

}
