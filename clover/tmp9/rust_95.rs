
use std::collections::HashMap;

fn check_dict_case(dict: HashMap<&str, &str>) -> bool {
    if dict.is_empty() {
        return false;
    }

    let mut all_lower = true;
    let mut all_upper = true;

    for key in dict.keys() {
        if key.chars().any(|c| !c.is_lowercase()) {
            all_lower = false;
        }
        if key.chars().any(|c| !c.is_uppercase()) {
            all_upper = false;
        }
    }

    all_lower || all_upper
}

fn main() {
    // Example usage:
    // let my_dict = HashMap::from([("key1", "value1"), ("key2", "value2")]);
    // println!("{}", check_dict_case(my_dict)); // Should print "true" or "false"
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