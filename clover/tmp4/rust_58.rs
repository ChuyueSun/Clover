
fn common(mut l1: Vec<i32>, mut l2: Vec<i32>) -> Vec<i32> {
    l1.sort_unstable();
    l2.sort_unstable();
    l1.dedup();
    l2.dedup();
    let mut common_elements = Vec::new();
    
    let mut l1_iter = l1.iter();
    let mut l2_iter = l2.iter();
    
    let mut l1_next = l1_iter.next();
    let mut l2_next = l2_iter.next();
    
    while let (Some(&val1), Some(&val2)) = (l1_next, l2_next) {
        match val1.cmp(&val2) {
            std::cmp::Ordering::Less => {
                l1_next = l1_iter.next();
            }
            std::cmp::Ordering::Greater => {
                l2_next = l2_iter.next();
            }
            std::cmp::Ordering::Equal => {
                common_elements.push(val1);
                l1_next = l1_iter.next();
                l2_next = l2_iter.next();
            }
        }
    }
    
    common_elements
}

fn main() {
    let l1 = vec![1, 2, 2, 5];
    let l2 = vec![2, 2, 3, 5, 6];
    let common_elements = common(l1, l2);
    println!("{:?}", common_elements); // Should print [2, 5]
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
