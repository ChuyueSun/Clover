
fn rolling_max(numbers: Vec<i32>) -> Vec<i32> {
    let mut rolling_max_vec = Vec::new();
    let mut current_max = i32::MIN;

    for &number in numbers.iter() {
        if number > current_max {
            current_max = number;
        }
        rolling_max_vec.push(current_max);
    }

    rolling_max_vec
}

fn main() {
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let result = rolling_max(numbers);
    println!("{:?}", result);
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
