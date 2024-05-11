
fn is_sorted(lst: Vec<i32>) -> bool {
    let mut prev = None;
    let mut counts = std::collections::HashMap::new();

    for &item in &lst {
        // Check if the current item is greater than the previous item
        if let Some(prev_value) = prev {
            if item < prev_value {
                // List is not strictly in ascending order
                return false;
            }
        }

        // Check for duplicates by counting occurrences
        let count = counts.entry(item).or_insert(0);
        *count += 1;
        if *count > 1 {
            // More than one duplicate of the same number found
            return false;
        }

        prev = Some(item);
    }

    true
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    println!("Is the list sorted and without more than one duplicate? {}", is_sorted(nums));
    // Output should be: Is the list sorted and without more than one duplicate? true
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
