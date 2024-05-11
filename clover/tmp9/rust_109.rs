
fn move_one_ball(arr: Vec<i32>) -> bool {
    if arr.is_empty() {
        return true;
    }
    
    let mut sorted = arr.clone();
    sorted.sort_unstable();
    
    for shift in 0..arr.len() {
        if arr.iter().skip(shift)
            .chain(arr.iter().take(shift))
            .eq(sorted.iter()) {
            return true;
        }
    }

    false
}

fn main() {
    // Test cases
    println!("{}", move_one_ball(vec![1, 3, 2])); // Example: Should return true because the array can be sorted after one right shift
    println!("{}", move_one_ball(vec![3, 1, 2])); // Example: Should return true because the array can be sorted after two right shifts
    println!("{}", move_one_ball(vec![])); // Example: Should return true because the array is empty
    println!("{}", move_one_ball(vec![1, 2, 4, 3])); // Example: Should return false because the array cannot be sorted with any amount of right shifts
    println!("{}", move_one_ball(vec![1, 5, 3, 4, 2])); // Example: Should return true because the array can be sorted after four right shifts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_one_ball() {
        assert!(move_one_ball(vec![3, 4, 5, 1, 2]) == true);
        assert!(move_one_ball(vec![3, 5, 10, 1, 2]) == true);
        assert!(move_one_ball(vec![4, 3, 1, 2]) == false);
        assert!(move_one_ball(vec![3, 5, 4, 1, 2]) == false);
        assert!(move_one_ball(vec![]) == true);
    }

}
