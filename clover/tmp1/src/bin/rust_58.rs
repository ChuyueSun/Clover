
fn common(l1: Vec<i32>, l2: Vec<i32>) -> Vec<i32> {
    let mut common_elements: Vec<i32> = l1
        .into_iter()
        .filter(|item| l2.contains(item))
        .collect();
    common_elements.sort_unstable();
    common_elements.dedup();
    common_elements
}

fn main() {
    let list1 = vec![1, 2, 2, 3, 4, 5];
    let list2 = vec![2, 3, 5, 6, 7];
    let result = common(list1, list2);
    println!("{:?}", result);
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
