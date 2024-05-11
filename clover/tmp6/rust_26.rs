
use std::collections::HashSet;

fn remove_duplicates(numbers: Vec<i32>) -> Vec<i32> {
    let mut seen = HashSet::new();
    let mut output = Vec::new();
    
    for &number in &numbers {
        if seen.insert(number) {
            output.push(number);
        }
    }
    
    output
}

fn main() {
    let numbers = vec![1, 2, 2, 3, 4, 4, 5];
    let result = remove_duplicates(numbers);
    println!("{:?}", result);
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
