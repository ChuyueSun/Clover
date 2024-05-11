
use std::collections::HashMap;

fn check_dict_case(dict: HashMap<&str, &str>) -> bool {
    if dict.is_empty() {
        return false;
    }

    let mut all_uppercase = true;
    let mut all_lowercase = true;

    for key in dict.keys() {
        if key.to_uppercase() != key {
            all_uppercase = false;
        }
        if key.to_lowercase() != key {
            all_lowercase = false;
        }
    }

    all_uppercase || all_lowercase
}

fn main() {
    // Example usage:
    // Create a dictionary (HashMap) and test the function
    // let dict = HashMap::from([("example", "value"), ("another", "entry")]);
    // println!("All keys match case: {}", check_dict_case(dict));
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
