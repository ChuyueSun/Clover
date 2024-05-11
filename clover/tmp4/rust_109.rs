
fn move_one_ball(mut arr: Vec<i32>) -> bool {
    if arr.is_empty() {
        return true;
    }

    let mut sorted_arr = arr.clone();
    sorted_arr.sort_unstable();

    for _ in 0..arr.len() {
        if arr == sorted_arr {
            return true;
        }
        // Perform a right shift operation on arr
        let last_element = arr.pop().unwrap();
        arr.insert(0, last_element);
    }

    false
}

fn main() {
    // Example usage:
    let arr = vec![3, 1, 2];
    println!("Is it possible to sort by shifting? {}", move_one_ball(arr));
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
