
fn move_one_ball(arr: Vec<i32>) -> bool {
    if arr.is_empty() {
        return true;
    }

    let len = arr.len();
    let mut sorted_arr = arr.clone();
    sorted_arr.sort_unstable();

    for i in 0..len {
        if sorted_arr == arr[i..].iter().chain(arr[..i].iter()).cloned().collect::<Vec<i32>>() {
            return true;
        }
    }

    false
}

fn main() {
    // You can test the function here
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
