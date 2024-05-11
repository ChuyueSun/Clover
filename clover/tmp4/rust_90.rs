
fn next_smallest(lst: Vec<i32>) -> Option<i32> {
    let mut sorted = lst.clone();
    sorted.sort_unstable();
    sorted.dedup();
    if sorted.len() >= 2 {
        Some(sorted[1])
    } else {
        None
    }
}

fn main() {
    let numbers = vec![4, 1, 6, 2, 2, 3];
    println!("{:?}", next_smallest(numbers)); // Example usage
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
