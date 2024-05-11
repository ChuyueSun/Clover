
fn move_one_ball(arr: Vec<i32>) -> bool {
    if arr.is_empty() {
        return true;
    }
    
    let mut potentials = arr.clone();

    for _ in 0..arr.len() {
        if potentials.windows(2).all(|w| w[0] <= w[1]) {
            return true;
        }
        let last = potentials.pop().unwrap();
        potentials.insert(0, last);
    }
    
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert!(move_one_ball(vec![]));
    }

    #[test]
    fn test_sorted() {
        assert!(move_one_ball(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_one_shift() {
        assert!(move_one_ball(vec![2, 3, 4, 5, 1]));
    }

    #[test]
    fn test_multiple_shifts() {
        assert!(move_one_ball(vec![3, 1, 2]));
    }

    #[test]
    fn test_not_possible() {
        assert!(!move_one_ball(vec![2, 1, 4, 3]));
    }
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
