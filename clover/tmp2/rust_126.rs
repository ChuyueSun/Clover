
fn is_sorted(lst: Vec<i32>) -> bool {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    let mut last_elem = None;

    for &item in &lst {
        if seen.contains(&item) {
            return false; // Duplicate found
        }
        if let Some(last) = last_elem {
            if item < last {
                return false; // Not sorted
            }
        }
        seen.insert(item);
        last_elem = Some(item);
    }

    true
}

fn main() {
    let numbers = vec![1, 2, 4, 4, 5];
    println!("Is sorted: {}", is_sorted(numbers));
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
