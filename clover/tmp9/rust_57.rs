
fn monotonic(l: Vec<i32>) -> bool {
    // A list with less than 2 elements is considered monotonic.
    if l.len() < 2 {
        return true;
    }

    // Ascertain the direction of monotonicity by comparing the first two different elements.
    let mut direction = 0; // 0 means no direction, 1 means increasing, -1 means decreasing
    for i in 1..l.len() {
        if l[i] > l[i - 1] {
            direction = 1;
            break;
        } else if l[i] < l[i - 1] {
            direction = -1;
            break;
        }
    }

    // Check if all the elements follow the ascertained direction
    for i in 1..l.len() {
        match direction {
            1 if l[i] < l[i - 1] => return false,
            -1 if l[i] > l[i - 1] => return false,
            _ => (),
        }
    }

    true
}

fn main() {
    // Example usage:
    assert_eq!(monotonic(vec![1, 2, 3, 4, 5]), true); // Increasing
    assert_eq!(monotonic(vec![5, 4, 3, 2, 1]), true); // Decreasing
    assert_eq!(monotonic(vec![1, 2, 2, 3]), true); // Neither, but still monotonic
    assert_eq!(monotonic(vec![1, 3, 2]), false); // Not monotonic
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monotonic() {
        assert!(monotonic(vec![1, 2, 4, 10]) == true);
        assert!(monotonic(vec![1, 2, 4, 20]) == true);
        assert!(monotonic(vec![1, 20, 4, 10]) == false);
        assert!(monotonic(vec![4, 1, 0, -10]) == true);
        assert!(monotonic(vec![4, 1, 1, 0]) == true);
        assert!(monotonic(vec![1, 2, 3, 2, 5, 60]) == false);
        assert!(monotonic(vec![1, 2, 3, 4, 5, 60]) == true);
        assert!(monotonic(vec![9, 9, 9, 9]) == true);
    }

}
