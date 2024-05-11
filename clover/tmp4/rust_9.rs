
fn rolling_max(numbers: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut max_so_far = i32::MIN;
    for &num in &numbers {
        if num > max_so_far {
            max_so_far = num;
        }
        result.push(max_so_far);
    }
    result
}

fn main() {
    let numbers = vec![1, 3, 2, 5, 3, 4, 8, 6];
    let rolling_maxes = rolling_max(numbers);
    println!("{:?}", rolling_maxes);
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
