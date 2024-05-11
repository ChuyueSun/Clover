
fn can_sort_by_right_shift(mut arr: Vec<i32>) -> bool {
    if arr.is_empty() {
        return true;
    }
    
    let mut sorted = arr.clone();
    sorted.sort_unstable(); // Sort the array to find the correct order.
    
    let n = arr.len();
    for i in 0..n {
        if arr == sorted {
            return true;
        }
        let last = arr.pop().expect("Array will not be empty");
        arr.insert(0, last); // Perform a right shift operation.
    }
    
    false
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
