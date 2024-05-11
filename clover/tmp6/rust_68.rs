
fn pluck(arr: Vec<i32>) -> Vec<i32> {
    let mut min_even = i32::MAX;
    let mut min_index = usize::MAX;

    for (index, &value) in arr.iter().enumerate() {
        if value % 2 == 0 && value < min_even {
            min_even = value;
            min_index = index;
        }
    }

    if min_even == i32::MAX {
        vec![]
    } else {
        vec![min_even, min_index as i32]
    }
}

fn main() {
    // Example usage:
    // Should print '[2, 1]' as 2 is the smallest even number and it's located at index 1.
    println!("{:?}", pluck(vec![3, 2, 1, 4]));

    // Should print '[]' as there are no even numbers. 
    println!("{:?}", pluck(vec![1, 3, 5]));
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
