fn prod_signs(arr: Vec<i32>) -> i32 {
    if arr.is_empty() {
        return 0; // Interpretation of 'None' as 0 for this scenario
    }

    let mut sign_product = 1;
    let mut magnitude_sum = 0;

    for &num in &arr {
        // Calculate the sum of magnitudes
        magnitude_sum += num.abs();

        // Update the sign product
        if num < 0 {
            sign_product *= -1;
        } else if num == 0 {
            sign_product = 0;
            break; // Since any number multiplied by zero is zero
        }
    }

    // Return the product of the magnitude sum and the sign product
    magnitude_sum * sign_product
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
