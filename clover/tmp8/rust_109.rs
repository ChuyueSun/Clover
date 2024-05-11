
fn move_one_ball(arr: Vec<i32>) -> bool {
    if arr.is_empty() {
        return true;
    }

    let mut candidates = arr
        .iter()
        .enumerate()
        .filter(|&(i, &val)| {
            if i == arr.len() - 1 {
                arr[0] >= val
            } else {
                arr[i + 1] >= val
            }
        })
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    candidates.retain(|&i| {
        let mut shifted_arr = arr.clone();
        shifted_arr.rotate_right(i + 1);
        shifted_arr.windows(2).all(|w| w[0] <= w[1])
    });

    !candidates.is_empty()
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
