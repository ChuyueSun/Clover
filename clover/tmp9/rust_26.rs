
use std::collections::HashMap;

fn remove_duplicates(numbers: Vec<i32>) -> Vec<i32> {
    let mut occurrences = HashMap::new();
    let mut unique_numbers = Vec::new();

    for &number in &numbers {
        let count = occurrences.entry(number).or_insert(0);
        *count += 1;
        if *count == 1 {
            unique_numbers.push(number);
        }
    }

    unique_numbers
        .into_iter()
        .filter(|&n| *occurrences.get(&n).unwrap() == 1)
        .collect()
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
