
fn common(mut l1: Vec<i32>, mut l2: Vec<i32>) -> Vec<i32> {
    // Sort both vectors first
    l1.sort_unstable();
    l2.sort_unstable();

    // Deduplicate both vectors
    l1.dedup();
    l2.dedup();

    // Find the intersection
    let mut common_elements = vec![];
    let mut l1_iter = l1.iter();
    let mut l2_iter = l2.iter();
    let mut l1_el = l1_iter.next();
    let mut l2_el = l2_iter.next();

    while let (Some(&x), Some(&y)) = (l1_el, l2_el) {
        match x.cmp(&y) {
            std::cmp::Ordering::Equal => {
                common_elements.push(x);
                l1_el = l1_iter.next();
                l2_el = l2_iter.next();
            }
            std::cmp::Ordering::Less => l1_el = l1_iter.next(),
            std::cmp::Ordering::Greater => l2_el = l2_iter.next(),
        }
    }

    common_elements
}

fn main() {
    let l1 = vec![1, 2, 2, 3, 4, 5];
    let l2 = vec![2, 3, 3, 4, 6];
    let result = common(l1, l2);
    println!("{:?}", result);  // This should print [2, 3, 4]
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
