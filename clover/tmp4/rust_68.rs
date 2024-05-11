
fn pluck(arr: Vec<i32>) -> Vec<i32> {
    let mut smallest_value = i32::MAX;
    let mut smallest_index = usize::MAX;

    for (index, &value) in arr.iter().enumerate() {
        if value % 2 == 0 && value < smallest_value {
            smallest_value = value;
            smallest_index = index;
        }
    }

    if smallest_value == i32::MAX {
        vec![]
    } else {
        vec![smallest_value, smallest_index as i32]
    }
}

fn main() {
    // Example usage:
    // let tree_branch = vec![3, 7, 4, 2, 5];
    // let plucked_node = pluck(tree_branch);
    // println!("{:?}", plucked_node); // Output: [2, 3]
}
`
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
