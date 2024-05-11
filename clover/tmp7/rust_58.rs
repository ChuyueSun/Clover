
fn common(mut l1: Vec<i32>, mut l2: Vec<i32>) -> Vec<i32> {
    l1.sort_unstable();
    l2.sort_unstable();
    l1.dedup();
    l2.dedup();
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < l1.len() && j < l2.len() {
        if l1[i] == l2[j] {
            result.push(l1[i]);
            i += 1;
            j += 1;
        } else if l1[i] < l2[j] {
            i += 1;
        } else {
            j += 1;
        }
    }
    result
}

fn main() {
    let l1 = vec![1, 2, 5, 2, 3];
    let l2 = vec![3, 2, 5, 7];
    let common_elements = common(l1, l2);
    println!("{:?}", common_elements); // Should print [2, 3, 5]
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
