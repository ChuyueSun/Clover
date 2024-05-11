
fn is_sorted(lst: Vec<i32>) -> bool {
    let mut prev = lst.get(0);
    let mut counts = std::collections::HashMap::new();

    for &item in &lst {
        if let Some(&count) = counts.get(&item) {
            if count >= 1 {
                return false;
            }
            counts.insert(item, count + 1);
        } else {
            counts.insert(item, 1);
        }
        
        if let Some(p) = prev {
            if item < p {
                return false;
            }
        }
        prev = Some(item);
    }
    true
}

fn main() {
    // Test cases
    println!("{}", is_sorted(vec![1,2,3,4,5])); // Should print true
    println!("{}", is_sorted(vec![1,1,2,3,4])); // Should print false
    println!("{}", is_sorted(vec![5,1,2,3,4])); // Should print false
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
