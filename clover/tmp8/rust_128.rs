
fn prod_signs(arr: Vec<i32>) -> Option<i32> {
    if arr.is_empty() {
        None
    } else {
        let mut sign_product = 1;
        let mut magnitude_sum = 0;
        for &num in &arr {
            sign_product *= num.signum();
            magnitude_sum += num.abs();
        }
        Some(magnitude_sum * sign_product)
    }
}

fn main() {
    // Example: if arr = [2, -3, 0, 4], the result should be -9 (sum of magnitudes is 9, sign product is -1)
    println!("{:?}", prod_signs(vec![2, -3, 0, 4])); // prints "Some(-9)"
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
