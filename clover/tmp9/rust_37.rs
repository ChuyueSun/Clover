
fn sort_even(mut nums: Vec<i32>) -> Vec<i32> {
    let mut even_indices_vals: Vec<i32> = nums.iter().enumerate()
                                              .filter(|&(i, _)| i % 2 == 0)
                                              .map(|(_, &val)| val)
                                              .collect();
    
    even_indices_vals.sort_unstable();

    for (i, val) in even_indices_vals.into_iter().enumerate() {
        nums[i * 2] = val;
    }
    
    nums
}

fn main() {
    // Example usage:
    let numbers = vec![5, 1, 3, 4, 2];
    println!("{:?}", sort_even(numbers)); // Prints the transformed list
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_sort_even() {
        assert_eq!(sort_even(vec![1, 2, 3]), vec![1, 2, 3]);
        assert_eq!(
            sort_even(vec![5, 3, -5, 2, -3, 3, 9, 0, 123, 1, -10]),
            vec![-10, 3, -5, 2, -3, 3, 5, 0, 9, 1, 123]
        );
        assert_eq!(
            sort_even(vec![5, 8, -12, 4, 23, 2, 3, 11, 12, -10]),
            vec![-12, 8, 3, 4, 5, 2, 12, 11, 23, -10]
        );
    }

}
