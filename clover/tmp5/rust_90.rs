
fn next_smallest(lst: Vec<i32>) -> Option<i32> {
    let mut sorted_lst = lst.clone();
    sorted_lst.sort_unstable();
    sorted_lst.dedup();
    if sorted_lst.len() < 2 {
        None
    } else {
        Some(sorted_lst[1])
    }
}

fn main() {
    // Example usage:
    let nums = vec![3, 1, 2, 1];
    match next_smallest(nums) {
        Some(second_smallest) => println!("The second smallest element is {}", second_smallest),
        None => println!("There is no second smallest element."),
    }
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
