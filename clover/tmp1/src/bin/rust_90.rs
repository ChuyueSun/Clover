
fn next_smallest(lst: Vec<i32>) -> Option<i32> {
    use std::collections::HashSet;
    let mut distinct_elements: HashSet<i32> = lst.iter().cloned().collect();
    
    match distinct_elements.len() {
        0 | 1 => None,
        _ => {
            let mut elements: Vec<i32> = distinct_elements.into_iter().collect();
            elements.sort();
            Some(elements[1])
        },
    }
}

fn main() {
    let list = vec![5, 3, 1, 3, 2];
    println!("{:?}", next_smallest(list)); // Should print Some(2)

    let list = vec![1];
    println!("{:?}", next_smallest(list)); // Should print None
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
