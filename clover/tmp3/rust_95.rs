
use std::collections::HashMap;

fn check_dict_case(dict: HashMap<&str, &str>) -> bool {
    if dict.is_empty() {
        return false;
    }

    let mut all_lowercase = true;
    let mut all_uppercase = true;

    for key in dict.keys() {
        if !key.chars().all(|c| c.is_lowercase()) {
            all_lowercase = false;
        }
        if !key.chars().all(|c| c.is_uppercase()) {
            all_uppercase = false;
        }
    }

    all_lowercase || all_uppercase
}

fn main() {
    // Example usage:
    // This dictionary has all keys in lowercase, so it should return true
    let dict_lowercase = HashMap::from([
        ("apple", "fruit"),
        ("banana", "fruit"),
        ("carrot", "vegetable"),
    ]);
    println!("All keys lowercase: {}", check_dict_case(dict_lowercase)); // should print true

    // This dictionary has all keys in uppercase, so it should return true
    let dict_uppercase = HashMap::from([
        ("APPLE", "fruit"),
        ("BANANA", "fruit"),
        ("CARROT", "vegetable"),
    ]);
    println!("All keys uppercase: {}", check_dict_case(dict_uppercase)); // should print true

    // This dictionary has mixed case keys, so it should return false
    let dict_mixed_case = HashMap::from([
        ("apple", "fruit"),
        ("BANANA", "fruit"),
        ("Carrot", "vegetable"),
    ]);
    println!("Mixed case keys: {}", check_dict_case(dict_mixed_case)); // should print false

    // This is an empty dictionary, so it should return false
    let dict_empty: HashMap<&str, &str> = HashMap::new();
    println!("Empty dictionary: {}", check_dict_case(dict_empty)); // should print false
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
