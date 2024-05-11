
fn min_sub_array_sum(nums: Vec<i64>) -> i64 {
    let mut min_ending_here = nums[0];
    let mut min_so_far = nums[0];
    
    for &num in nums.iter().skip(1) {
        min_ending_here = i64::min(num, min_ending_here + num);
        min_so_far = i64::min(min_so_far, min_ending_here);
    }
    
    min_so_far
}

fn main() {
    let nums = vec![3, -4, 2, -3];
    let result = min_sub_array_sum(nums);
    println!("The minimum sum of any non-empty sub-array is: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_sub_array_sum() {
        assert!(min_sub_array_sum(vec![2, 3, 4, 1, 2, 4]) == 1);
        assert!(min_sub_array_sum(vec![-1, -2, -3]) == -6);
        assert!(min_sub_array_sum(vec![-1, -2, -3, 2, -10]) == -14);
        assert!(min_sub_array_sum(vec![-9999999999999999]) == -9999999999999999);
        assert!(min_sub_array_sum(vec![0, 10, 20, 1000000]) == 0);
        assert!(min_sub_array_sum(vec![-1, -2, -3, 10, -5]) == -6);
        assert!(min_sub_array_sum(vec![100, -1, -2, -3, 10, -5]) == -6);
        assert!(min_sub_array_sum(vec![10, 11, 13, 8, 3, 4]) == 3);
        assert!(min_sub_array_sum(vec![100, -33, 32, -1, 0, -2]) == -33);
        assert!(min_sub_array_sum(vec![-10]) == -10);
        assert!(min_sub_array_sum(vec![7]) == 7);
        assert!(min_sub_array_sum(vec![1, -1]) == -1);
    }

}
