
fn prod_signs(arr: Vec<i32>) -> Option<i32> {
    if arr.is_empty() {
        return None;
    }

    let product_of_signs: i32 = arr.iter().map(|&num| num.signum()).product();
    let sum_of_magnitudes: i32 = arr.iter().map(|&num| num.abs()).sum();
    
    Some(sum_of_magnitudes * product_of_signs)
}

fn main() {
    // Example usage:
    let numbers = vec![-1, 2, -3, 4];
    println!("{:?}", prod_signs(numbers)); // Should output Some(10)
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
