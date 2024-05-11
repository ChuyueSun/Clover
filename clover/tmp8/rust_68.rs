
fn pluck(arr: Vec<i32>) -> Vec<i32> {
    let mut plucked_node = Vec::new();

    if let Some((index, &value)) = arr.iter()
        .enumerate()
        .filter(|&(_, &v)| v % 2 == 0)
        .min_by_key(|&(_, &v)| v)
    {
        plucked_node.push(value);
        plucked_node.push(index as i32);
    }

    plucked_node
}

fn main() {
    // Example usage:
    let arr = vec![5, 3, 2, 4, 1];
    println!("{:?}", pluck(arr)); // Output will be [2, 2]
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
