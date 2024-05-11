
fn common(mut l1: Vec<i32>, mut l2: Vec<i32>) -> Vec<i32> {
    // Sort both vectors
    l1.sort_unstable();
    l2.sort_unstable();

    // Get the intersection of the two vectors
    let mut common_elements: Vec<i32> = l1
        .iter()
        .filter(|&item| l2.binary_search(item).is_ok())
        .cloned()
        .collect();

    // Deduplicate the common elements
    common_elements.dedup();
    
    common_elements
}

fn main() {
    let v1 = vec![3, 2, 5, 5, 3];
    let v2 = vec![5, 3, 3, 4, 5];
    let result = common(v1, v2);
    println!("{:?}", result);  // Example output: [3, 5], oder may vary
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common() {
        assert!(
            common(vec![1, 4, 3, 34, 653, 2, 5], vec![5, 7, 1, 5, 9, 653, 121]) == vec![1, 5, 653]
        );
        assert!(common(vec![5, 3, 2, 8], vec![3, 2]) == vec![2, 3]);
        assert!(common(vec![4, 3, 2, 8], vec![3, 2, 4]) == vec![2, 3, 4]);
        assert!(common(vec![4, 3, 2, 8], vec![]) == vec![]);
    }

}
