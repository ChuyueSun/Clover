
fn next_smallest(lst: Vec<i32>) -> Option<i32> {
    let mut unique_sorted = lst.into_iter().collect::<std::collections::HashSet<_>>().into_iter().collect::<Vec<_>>();
    unique_sorted.sort();
    unique_sorted.get(1).copied()
}

fn main() {
    let numbers = vec![4, 3, 2, 1, 3, 4, 2];
    println!("The 2nd smallest number is {:?}", next_smallest(numbers));
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
