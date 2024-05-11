
fn common(mut l1: Vec<i32>, mut l2: Vec<i32>) -> Vec<i32> {
    l1.sort_unstable();
    l2.sort_unstable();
    l1.dedup();
    l2.dedup();
    let mut result = Vec::new();
    
    let mut l1_iter = l1.iter();
    let mut l2_iter = l2.iter();
    let mut l1_elem = l1_iter.next();
    let mut l2_elem = l2_iter.next();
    
    while l1_elem.is_some() && l2_elem.is_some() {
        match (l1_elem, l2_elem) {
            (Some(&x), Some(&y)) if x == y => {
                result.push(x);
                l1_elem = l1_iter.next();
                l2_elem = l2_iter.next();
            },
            (Some(&x), Some(&y)) if x < y => {
                l1_elem = l1_iter.next();
            },
            (Some(&x), Some(&y)) if x > y => {
                l2_elem = l2_iter.next();
            },
            _ => break,
        }
    }
    
    result
}

fn main() {
    // Example usage:
    let list1 = vec![1, 2, 2, 3, 4];
    let list2 = vec![2, 3, 5, 6];
    let common_elements = common(list1, list2);
    println!("{:?}", common_elements); // Should print [2, 3]
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
