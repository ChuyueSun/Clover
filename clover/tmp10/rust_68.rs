
fn pluck(arr: Vec<i32>) -> Vec<i32> {
    arr.iter()
        .enumerate()
        .filter(|&(_idx, &val)| val % 2 == 0 && val >= 0)
        .min_by_key(|&(idx, &val)| (val, idx))
        .map_or(Vec::new(), |(idx, &val)| vec![val, idx as i32])
}

fn main() {
    // Example usage:
    let branch = vec![1, 3, 4, 6, 7, 8];
    let plucked_node = pluck(branch);
    println!("{:?}", plucked_node); // Should print [4, 2]
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
