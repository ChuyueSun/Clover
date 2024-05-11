
use std::collections::HashMap;

fn check_dict_case(dict: HashMap<&str, &str>) -> bool {
    if dict.is_empty() {
        return false;
    }

    let mut all_lowercase = true;
    let mut all_uppercase = true;

    for key in dict.keys() {
        if !key.chars().all(char::is_lowercase) {
            all_lowercase = false;
        }
        if !key.chars().all(char::is_uppercase) {
            all_uppercase = false;
        }
        if !all_lowercase && !all_uppercase {
            return false;
        }
    }

    true
}

fn main() {
    // Example usage
    let mut dict = HashMap::new();
    dict.insert("example", "value");
    dict.insert("test", "value");
    println!("{}", check_dict_case(dict)); // Should print 'true'

    let mut dict_upper = HashMap::new();
    dict_upper.insert("EXAMPLE", "value");
    dict_upper.insert("TEST", "value");
    println!("{}", check_dict_case(dict_upper)); // Should print 'true'

    let mut mixed_dict = HashMap::new();
    mixed_dict.insert("example", "value");
    mixed_dict.insert("TEST", "value");
    println!("{}", check_dict_case(mixed_dict)); // Should print 'false'

    let empty_dict: HashMap<&str, &str> = HashMap::new();
    println!("{}", check_dict_case(empty_dict)); // Should print 'false'
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
