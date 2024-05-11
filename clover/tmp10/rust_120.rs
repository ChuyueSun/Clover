
fn maximum_k(arr: Vec<i32>, k: usize) -> Vec<i32> {
    let mut heap = std::collections::BinaryHeap::from(arr);
    let mut result = Vec::with_capacity(k);
    
    for _ in 0..k {
        if let Some(max_val) = heap.pop() {
            result.push(max_val);
        }
    }
    
    result.sort_unstable();
    result
}

fn main() {
    let arr = vec![5, 2, 9, 3, 6, 7];
    let k = 3;
    let top_k = maximum_k(arr, k);
    println!("{:?}", top_k); // The top 3 maximum numbers in this case would be [5, 6, 7, 9]
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
