
fn remove_duplicates(numbers: Vec<i32>) -> Vec<i32> {
    let mut unique_numbers = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for &num in &numbers {
        if !seen.contains(&num) {
            unique_numbers.push(num);
            seen.insert(num);
        }
    }

    unique_numbers
}

fn main() {
    let numbers = vec![1, 2, 2, 3, 4, 4, 5];
    let without_duplicates = remove_duplicates(numbers);

    println!("Numbers without duplicates: {:?}", without_duplicates);
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
