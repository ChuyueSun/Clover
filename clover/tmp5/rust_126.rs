
fn is_sorted(lst: Vec<i32>) -> bool {
    let mut prev = None;
    let mut counts = std::collections::HashMap::new();

    for &item in &lst {
        // Count the occurrences of each number
        let counter = counts.entry(item).or_insert(0);
        *counter += 1;
        if *counter > 1 {
            return false;
        }

        // Check if the list is sorted
        if let Some(prev_value) = prev {
            if item < prev_value {
                return false;
            }
        }
        prev = Some(item);
    }
    
    true
}

fn main() {
    // Examples
    assert_eq!(is_sorted(vec![1, 2, 3, 4, 5]), true);
    assert_eq!(is_sorted(vec![1, 2, 2, 3, 4]), false);
    assert_eq!(is_sorted(vec![5, 3, 1]), false);
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
