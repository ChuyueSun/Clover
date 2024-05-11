
fn is_sorted(lst: Vec<i32>) -> bool {
    let mut count_map = std::collections::HashMap::new();
    
    for &item in &lst {
        *count_map.entry(item).or_insert(0) += 1;
        if count_map[&item] > 1 {
            return false;
        }
    }
    
    lst.windows(2).all(|window| window[0] <= window[1])
}

fn main() {
    // Example usage:
    // Should return true because it is sorted and has no duplicates
    println!("{}", is_sorted(vec![1, 2, 3, 4, 5])); 
    // Should return false because there are duplicates
    println!("{}", is_sorted(vec![1, 2, 2, 3, 4])); 
    // Should return false because it is not sorted
    println!("{}", is_sorted(vec![5, 3, 1])); 
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
