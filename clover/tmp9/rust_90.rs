
fn next_smallest(lst: Vec<i32>) -> Option<i32> {
    use std::collections::HashSet;
    let mut unique_elements: HashSet<i32> = lst.into_iter().collect();
    // Remove the smallest element, if it exists.
    if let Some(&min) = unique_elements.iter().min() {
        unique_elements.remove(&min);
    }
    // Return the next smallest element, if it exists.
    unique_elements.into_iter().min()
}

fn main() {
    // Example usage:
    let numbers = vec![5, 3, 2, 5, 4, 2];
    println!("{:?}", next_smallest(numbers)); // Output should be Some(3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_smallest() {
        assert!(next_smallest(vec![1, 2, 3, 4, 5]) == 2);
        assert!(next_smallest(vec![5, 1, 4, 3, 2]) == 2);
        assert!(next_smallest(vec![]) == -1);
        assert!(next_smallest(vec![1, 1]) == -1);
        assert!(next_smallest(vec![1, 1, 1, 1, 0]) == 1);
        assert!(next_smallest(vec![-35, 34, 12, -45]) == -35);
    }

}
