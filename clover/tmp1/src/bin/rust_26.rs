
fn remove_duplicates(numbers: Vec<i32>) -> Vec<i32> {
    let mut seen = std::collections::HashSet::new();
    let mut unique = Vec::new();
    
    for &number in &numbers {
        if !seen.contains(&number) {
            seen.insert(number);
            unique.push(number);
        }
    }
    
    unique
}

fn main() {
    let numbers = vec![1, 2, 2, 3, 4, 4, 5, 1];
    let numbers_no_duplicates = remove_duplicates(numbers);
    println!("{:?}", numbers_no_duplicates); // Should print [1, 2, 3, 4, 5]
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
