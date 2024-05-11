
fn common(l1: Vec<i32>, l2: Vec<i32>) -> Vec<i32> {
    let set1: std::collections::HashSet<_> = l1.into_iter().collect();
    let set2: std::collections::HashSet<_> = l2.into_iter().collect();
    
    let mut common_elements: Vec<_> = set1.intersection(&set2).cloned().collect();
    common_elements.sort();
    common_elements
}

fn main() {
    let l1 = vec![4, 2, 3, 4];
    let l2 = vec![1, 2, 3, 4];
    
    let common_elements = common(l1, l2);
    println!("{:?}", common_elements); // Output will be a vector with sorted unique common elements
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
