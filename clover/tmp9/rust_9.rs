
fn rolling_max(numbers: Vec<i32>) -> Vec<i32> {
    numbers
        .iter()
        .scan(i32::MIN, |state, &x| {
            if x > *state {
                *state = x;
            }
            Some(*state)
        })
        .collect()
}

fn main() {
    // Example usage:
    let numbers = vec![1, 3, 2, 5, 4, 8, 2, 9, 0];
    let roll_max = rolling_max(numbers);
    println!("{:?}", roll_max);
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_rolling_max() {
        assert!(rolling_max(vec![]) == vec![]);
        assert!(rolling_max(vec![1, 2, 3, 4]) == vec![1, 2, 3, 4]);
        assert!(rolling_max(vec![4, 3, 2, 1]) == vec![4, 4, 4, 4]);
        assert!(rolling_max(vec![3, 2, 3, 100, 3]) == vec![3, 3, 3, 100, 100]);
    }

}
