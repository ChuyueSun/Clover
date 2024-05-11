
use std::collections::HashMap;

fn check_dict_case(dict: HashMap<&str, &str>) -> bool {
    if dict.is_empty() {
        return false;
    }

    let mut all_lower = true;
    let mut all_upper = true;

    for key in dict.keys() {
        if !key.chars().all(|c| c.is_lowercase()) {
            all_lower = false;
        }
        if !key.chars().all(|c| c.is_uppercase()) {
            all_upper = false;
        }
    }

    all_lower || all_upper
}

fn main() {
    // Example usage:
    let dict_lower = HashMap::from([
        ("apple", "fruit"),
        ("banana", "fruit"),
    ]);
    assert_eq!(check_dict_case(dict_lower), true);

    let dict_upper = HashMap::from([
        ("APPLE", "FRUIT"),
        ("BANANA", "FRUIT"),
    ]);
    assert_eq!(check_dict_case(dict_upper), true);

    let dict_mixed = HashMap::from([
        ("APPLE", "FRUIT"),
        ("banana", "fruit"),
    ]);
    assert_eq!(check_dict_case(dict_mixed), false);

    let dict_empty: HashMap<&str, &str> = HashMap::new();
    assert_eq!(check_dict_case(dict_empty), false);
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
