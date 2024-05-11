
fn maximum_120(arr: Vec<i32>, k: i32) -> Vec<i32> {
    let mut sorted_arr = arr.clone();
    sorted_arr.sort_unstable_by(|a, b| b.cmp(a));
    sorted_arr.truncate(k as usize);
    sorted_arr
}

fn main() {
    // Example usage:
    let arr = vec![3, 5, 2, 9, 1];
    let k = 3;
    let result = maximum_120(arr, k);
    println!("{:?}", result); // Output will be [9, 5, 3]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_120() {
        assert_eq!(maximum_120(vec![-3, -4, 5], 3), vec![-4, -3, 5]);
        assert_eq!(maximum_120(vec![4, -4, 4], 2), vec![4, 4]);
        assert_eq!(maximum_120(vec![-3, 2, 1, 2, -1, -2, 1], 1), vec![2]);
        assert_eq!(
            maximum_120(vec![123, -123, 20, 0, 1, 2, -3], 3),
            vec![2, 20, 123]
        );
        assert_eq!(
            maximum_120(vec![-123, 20, 0, 1, 2, -3], 4),
            vec![0, 1, 2, 20]
        );
        assert_eq!(
            maximum_120(vec![5, 15, 0, 3, -13, -8, 0], 7),
            vec![-13, -8, 0, 0, 3, 5, 15]
        );
        assert_eq!(maximum_120(vec![-1, 0, 2, 5, 3, -10], 2), vec![3, 5]);
        assert_eq!(maximum_120(vec![1, 0, 5, -7], 1), vec![5]);
        assert_eq!(maximum_120(vec![4, -4], 2), vec![-4, 4]);
        assert_eq!(maximum_120(vec![-10, 10], 2), vec![-10, 10]);
        assert_eq!(maximum_120(vec![1, 2, 3, -23, 243, -400, 0], 0), vec![]);
    }

}
