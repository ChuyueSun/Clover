
fn rolling_max(numbers: Vec<i32>) -> Vec<i32> {
    let mut max_rolling = Vec::with_capacity(numbers.len());
    let mut current_max = i32::MIN;

    for &number in &numbers {
        if number > current_max {
            current_max = number;
        }
        max_rolling.push(current_max);
    }

    max_rolling
}

fn main() {
    let numbers = vec![1, 3, 2, 5, 4];
    let max_numbers = rolling_max(numbers);
    println!("{:?}", max_numbers);
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
