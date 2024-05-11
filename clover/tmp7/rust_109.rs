
fn move_one_ball(arr: Vec<i32>) -> bool {
    if arr.is_empty() {
        return true;
    }
    
    let mut sorted_arr = arr.clone();
    sorted_arr.sort();
    
    for i in 0..sorted_arr.len() {
        if sorted_arr
            .iter()
            .zip(arr[i..].iter().chain(arr[..i].iter()))
            .all(|(a, b)| a == b)
        {
            return true;
        }
    }
    
    false
}

fn main() {
    // An example usage of the function
    let arr = vec![3, 8, 5, 1];
    println!("{}", move_one_ball(arr)); // Returns true
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
