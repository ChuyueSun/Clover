
fn next_smallest(lst: Vec<i32>) -> Option<i32> {
    let mut unique_sorted_lst = lst.clone();
    unique_sorted_lst.sort_unstable();
    unique_sorted_lst.dedup();
    if unique_sorted_lst.len() > 1 {
        Some(unique_sorted_lst[1])
    } else {
        None
    }
}

fn main() {
    // Example usage:
    // Given list with at least two distinct elements
    let lst_1 = vec![5, 2, 3, 3, 4];
    println!("{:?}", next_smallest(lst_1)); // Should print Some(3)

    // Given list with no second smallest element
    let lst_2 = vec![1];
    println!("{:?}", next_smallest(lst_2)); // Should print None

    // Given list with two elements including negative integers
    let lst_3 = vec![-2, -3];
    println!("{:?}", next_smallest(lst_3)); // Should print Some(-2)

    // Given empty list
    let lst_4: Vec<i32> = vec![];
    println!("{:?}", next_smallest(lst_4)); // Should print None
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
