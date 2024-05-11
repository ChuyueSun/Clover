
use std::collections::HashSet;

fn remove_duplicates(numbers: Vec<i32>) -> Vec<i32> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();

    for number in numbers {
        if seen.insert(number) {
            result.push(number);
        }
    }

    result
}

fn main() {
    let numbers = vec![1, 2, 2, 3, 4, 2, 3, 5];
    let unique_numbers = remove_duplicates(numbers);
    println!("{:?}", unique_numbers);
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
