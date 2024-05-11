
fn next_smallest(lst: Vec<i32>) -> Option<i32> {
    let mut sorted_list = lst.clone();
    sorted_list.sort_unstable();
    sorted_list.dedup();
    if sorted_list.len() > 1 {
        Some(sorted_list[1])
    } else {
        None
    }
}

fn main() {
    let nums = vec![3, 1, 2, 4, 1];
    match next_smallest(nums) {
        Some(second_smallest) => println!("The 2nd smallest number is {}", second_smallest),
        None => println!("There is no 2nd smallest number."),
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
