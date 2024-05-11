 the Rust program that implements the functionality described in the docstring:

```rust
fn rolling_max(numbers: Vec<i32>) -> Vec<i32> {
    let mut max_so_far = std::i32::MIN;
    let mut rolling_maxes = Vec::with_capacity(numbers.len());
    
    for &number in &numbers {
        if number > max_so_far {
            max_so_far = number;
        }
        rolling_maxes.push(max_so_far);
    }
    
    rolling_maxes
}

fn main() {
    let numbers = vec![1, 5, 2, 3, 4, 8, 2, 1, 5];
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
