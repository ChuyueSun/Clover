
fn prod_signs(arr: Vec<i32>) -> Option<i32> {
    if arr.is_empty() {
        return None;
    }

    let mut product_of_signs = 1;
    let mut sum_of_magnitudes = 0;

    for &number in &arr {
        product_of_signs *= number.signum();
        sum_of_magnitudes += number.abs();
    }

    Some(sum_of_magnitudes * product_of_signs)
}

fn main() {
    // Example usage:
    let numbers = vec![-1, 2, -3, 4];
    match prod_signs(numbers) {
        Some(result) => println!("Result: {}", result),
        None => println!("Input array was empty"),
    }
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
