
fn next_smallest(lst: Vec<i32>) -> Option<i32> {
    use std::collections::HashSet;
    let mut unique_numbers = lst.into_iter().collect::<HashSet<i32>>();
    
    unique_numbers.remove(&unique_numbers.iter().min()?);
    unique_numbers.iter().min().cloned()
}

fn main() {
    let numbers = vec![3, 1, 4, 1, 5, 9];
    match next_smallest(numbers) {
        Some(value) => println!("The 2nd smallest element is {}", value),
        None => println!("There is no 2nd smallest element."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_smallest() {
        assert!(next_smallest(vec![1, 2, 3, 4, 5]) == 2);
        assert!(next_smallest(vec![5, 1, 4, 3, 2]) == 2);
        assert!(next_smallest(vec![]) == -1);
        assert!(next_smallest(vec![1, 1]) == -1);
        assert!(next_smallest(vec![1, 1, 1, 1, 0]) == 1);
        assert!(next_smallest(vec![-35, 34, 12, -45]) == -35);
    }

}
