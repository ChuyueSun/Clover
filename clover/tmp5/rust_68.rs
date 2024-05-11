
fn pluck(arr: Vec<i32>) -> Vec<i32> {
    if let Some((index, &value)) = arr.iter()
                                      .enumerate()
                                      .filter(|&(_, &v)| v % 2 == 0)
                                      .min_by_key(|&(_, &v)| v) {
        vec![value, index as i32]
    } else {
        Vec::new()
    }
}

fn main() {
    // Example usage:
    let nodes = vec![1, 3, 4, 6, 8, 10, 2];
    let plucked_node = pluck(nodes);
    println!("{:?}", plucked_node); // Should print the smallest even number with index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pluck() {
        assert!(pluck(vec![4, 2, 3]) == vec![2, 1]);
        assert!(pluck(vec![1, 2, 3]) == vec![2, 1]);
        assert!(pluck(vec![]) == vec![]);
        assert!(pluck(vec![5, 0, 3, 0, 4, 2]) == vec![0, 1]);
        assert!(pluck(vec![1, 2, 3, 0, 5, 3]) == vec![0, 3]);
        assert!(pluck(vec![5, 4, 8, 4, 8]) == vec![4, 1]);
        assert!(pluck(vec![7, 6, 7, 1]) == vec![6, 1]);
        assert!(pluck(vec![7, 9, 7, 1]) == vec![]);
    }

}
