 the Rust program that implements the functionality described in the docstring:

```rust
fn remove_duplicates(numbers: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut occurrences = std::collections::HashSet::new();
    
    for &number in &numbers {
        if occurrences.insert(number) {
            result.push(number);
        }
    }
    
    result
}

fn main() {
    let numbers = vec![1, 2, 2, 3, 4, 4, 5, 6, 6];
    println!("{:?}", remove_duplicates(numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_remove_duplicates() {
        assert!(remove_duplicates(vec![]) == []);
        assert!(remove_duplicates(vec![1, 2, 3, 4]) == vec![1, 2, 3, 4]);
        assert!(remove_duplicates(vec![1, 2, 3, 2, 4, 3, 5]) == [1, 4, 5]);
    }

}
