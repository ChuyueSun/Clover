
fn move_one_ball(arr: Vec<i32>) -> bool {
    if arr.is_empty() {
        return true;
    }
    
    let mut expected = arr.clone();
    expected.sort_unstable();
    
    for i in 0..arr.len() {
        if arr[i..].iter().chain(arr[..i].iter()).eq(expected.iter()) {
            return true;
        }
    }

    false
}

fn main() {
    // Example usage:
    let arr = vec![3, 1, 2];
    println!("Can be sorted by right shifts: {}", move_one_ball(arr));
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
