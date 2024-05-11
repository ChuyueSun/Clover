
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
        // As soon as we know that the keys aren't all upper or aren't all lower, we can exit
        if !all_lower && !all_upper {
            return false;
        }
    }

    all_lower || all_upper
}

fn main() {
    // Example usage:
    let mut dict = HashMap::new();
    dict.insert("example", "value");
    dict.insert("test", "value");
    println!("{}", check_dict_case(dict)); // Expected output: true

    let mut dict_upper = HashMap::new();
    dict_upper.insert("EXAMPLE", "value");
    dict_upper.insert("TEST", "value");
    println!("{}", check_dict_case(dict_upper)); // Expected output: true

    let mut dict_mixed = HashMap::new();
    dict_mixed.insert("Example", "value");
    dict_mixed.insert("test", "value");
    println!("{}", check_dict_case(dict_mixed)); // Expected output: false

    let empty_dict: HashMap<&str, &str> = HashMap::new();
    println!("{}", check_dict_case(empty_dict)); // Expected output: false
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
