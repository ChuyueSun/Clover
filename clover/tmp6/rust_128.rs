
fn prod_signs(arr: Vec<i32>) -> Option<i32> {
    if arr.is_empty() {
        None
    } else {
        let product_of_signs: i32 = arr.iter().map(|&x| x.signum()).product();
        let sum_of_magnitudes: i32 = arr.iter().map(|&x| x.abs()).sum();
        Some(sum_of_magnitudes * product_of_signs)
    }
}

fn main() {
    // Test cases
    println!("{:?}", prod_signs(vec![1, -2, 3]));  // Output: Some(4)
    println!("{:?}", prod_signs(vec![]));          // Output: None
    println!("{:?}", prod_signs(vec![1, 0, -1]));  // Output: Some(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prod_signs() {
        assert_eq!(prod_signs(vec![1, 2, 2, -4]), -9);
        assert_eq!(prod_signs(vec![0, 1]), 0);
        assert_eq!(prod_signs(vec![1, 1, 1, 2, 3, -1, 1]), -10);
        assert_eq!(prod_signs(vec![]), -32768);
        assert_eq!(prod_signs(vec![2, 4, 1, 2, -1, -1, 9]), 20);
        assert_eq!(prod_signs(vec![-1, 1, -1, 1]), 4);
        assert_eq!(prod_signs(vec![-1, 1, 1, 1]), -4);
        assert_eq!(prod_signs(vec![-1, 1, 1, 0]), 0);
    }

}
