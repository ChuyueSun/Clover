
fn prod_signs(arr: Vec<i32>) -> Option<i32> {
    if arr.is_empty() {
        None
    } else {
        let mut magnitude_sum = 0;
        let mut sign_product = 1;
        for &num in &arr {
            magnitude_sum += num.abs();
            sign_product *= match num {
                n if n > 0 => 1,
                n if n < 0 => -1,
                _ => 0,
            };
        }
        Some(magnitude_sum * sign_product)
    }
}

fn main() {
    // Example Usage:
    let numbers = vec![-1, 2, -3, 4];
    println!("{:?}", prod_signs(numbers)); // Outputs: Some(-10)
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
