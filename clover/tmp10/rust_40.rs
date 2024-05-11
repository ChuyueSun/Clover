
fn triples_sum_to_zero(nmbs: Vec<i32>) -> bool {
    let mut nmbs = nmbs;
    nmbs.sort_unstable();
    let len = nmbs.len();

    for i in 0..len {
        let mut left = i + 1;
        let mut right = len - 1;
        
        while left < right {
            let sum = nmbs[i] + nmbs[left] + nmbs[right];
            
            if sum == 0 {
                return true;
            } else if sum < 0 {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    false
}

fn main() {
    let nmbs = vec![-1, 0, 1, 2, -1, -4];
    println!("Contains triple that sum to zero: {}", triples_sum_to_zero(nmbs));
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
