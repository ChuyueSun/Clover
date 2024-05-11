
fn next_smallest(lst: Vec<i32>) -> Option<i32> {
    use std::collections::HashSet;
    let mut set: HashSet<i32> = HashSet::new();
    for &num in &lst {
        set.insert(num);
    }
    let mut unique_nums: Vec<i32> = set.into_iter().collect();
    if unique_nums.len() < 2 {
        None
    } else {
        unique_nums.sort();
        Some(unique_nums[1])
    }
}

fn main() {
    // You can test the function here
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
