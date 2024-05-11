
fn is_sorted(lst: Vec<i32>) -> bool {
    let mut count_map = std::collections::HashMap::new();

    for &item in &lst {
        // Check for duplicates
        let count = count_map.entry(item).or_insert(0);
       if *count > 0 {
           return false;
       }
       *count += 1;
    
        // Check for ascending order
        if *count > 1 {
            return false;
        }
    }
    lst.windows(2).all(|w| w[0] <= w[1])
}

fn main() {
    // Example usage:
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Are numbers sorted and with no more than one duplicate? {}", is_sorted(numbers));
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
