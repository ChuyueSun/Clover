
fn triples_sum_to_zero(nums: Vec<i32>) -> bool {
    let n = nums.len();
    for i in 0..n {
        for j in i+1..n {
            for k in j+1..n {
                if nums[i] + nums[j] + nums[k] == 0 {
                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    // Example usage:
    let nums = vec![-1, 0, 1, 2, -1, -4];
    println!("Does the list contain three elements that sum to zero? {}", triples_sum_to_zero(nums));
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_triples_sum_to_zero() {
        assert!(triples_sum_to_zero(vec![1, 3, 5, 0]) == false);
        assert!(triples_sum_to_zero(vec![1, 3, 5, -1]) == false);
        assert!(triples_sum_to_zero(vec![1, 3, -2, 1]) == true);
        assert!(triples_sum_to_zero(vec![1, 2, 3, 7]) == false);
        assert!(triples_sum_to_zero(vec![1, 2, 5, 7]) == false);
        assert!(triples_sum_to_zero(vec![2, 4, -5, 3, 9, 7]) == true);
        assert!(triples_sum_to_zero(vec![1]) == false);
        assert!(triples_sum_to_zero(vec![1, 3, 5, -100]) == false);
        assert!(triples_sum_to_zero(vec![100, 3, 5, -100]) == false);
    }

}
