
fn is_sorted(lst: Vec<i32>) -> bool {
    if lst.is_empty() {
        return true;
    }

    let mut last_value = lst[0];
    let mut seen_values = std::collections::HashSet::new();
    
    for &value in lst.iter() {
        if value < last_value || (value == last_value && !seen_values.insert(value)) {
            return false;
        }
        last_value = value;
    }

    true
}

fn main() {
    // Example usage:
    let numbers = vec![1, 2, 2, 3, 4, 5];
    println!("Are the numbers sorted and unique? {}", is_sorted(numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_sorted() {
        assert_eq!(is_sorted(vec![5]), true);
        assert_eq!(is_sorted(vec![1, 2, 3, 4, 5]), true);
        assert_eq!(is_sorted(vec![1, 3, 2, 4, 5]), false);
        assert_eq!(is_sorted(vec![1, 2, 3, 4, 5, 6]), true);
        assert_eq!(is_sorted(vec![1, 2, 3, 4, 5, 6, 7]), true);
        assert_eq!(is_sorted(vec![1, 3, 2, 4, 5, 6, 7]), false);
        assert_eq!(is_sorted(vec![]), true);
        assert_eq!(is_sorted(vec![1]), true);
        assert_eq!(is_sorted(vec![3, 2, 1]), false);
        assert_eq!(is_sorted(vec![1, 2, 2, 2, 3, 4]), false);
        assert_eq!(is_sorted(vec![1, 2, 3, 3, 3, 4]), false);
        assert_eq!(is_sorted(vec![1, 2, 2, 3, 3, 4]), true);
        assert_eq!(is_sorted(vec![1, 2, 3, 4]), true);
    }


}
