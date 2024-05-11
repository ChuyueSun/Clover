
fn rolling_max(numbers: Vec<i32>) -> Vec<i32> {
    let mut results = Vec::new();
    let mut max_so_far = i32::MIN;
    
    for &number in &numbers {
        if number > max_so_far {
            max_so_far = number;
        }
        results.push(max_so_far);
    }
    
    results
}

fn main() {
    let numbers = vec![1, 3, 6, 2, 5];
    let rolling_maximums = rolling_max(numbers);
    println!("{:?}", rolling_maximums);
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
