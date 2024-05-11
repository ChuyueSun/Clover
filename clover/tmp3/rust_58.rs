
fn common(l1: Vec<i32>, l2: Vec<i32>) -> Vec<i32> {
    use std::collections::HashSet;
    let set1: HashSet<i32> = l1.into_iter().collect();
    let set2: HashSet<i32> = l2.into_iter().collect();

    let mut common_elements: Vec<i32> = set1.intersection(&set2).cloned().collect();
    common_elements.sort_unstable();
    common_elements
}

fn main() {
    let list1 = vec![1, 2, 2, 3];
    let list2 = vec![2, 3, 4];
    let result = common(list1, list2);
    println!("{:?}", result); // Expected: [2, 3]
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
