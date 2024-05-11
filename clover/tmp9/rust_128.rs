
fn prod_signs(arr: Vec<i32>) -> Option<i32> {
    if arr.is_empty() {
        return None;
    }

    let product_of_signs = arr.iter().map(|&num| num.signum()).product::<i32>();
    let magnitude_sum: i32 = arr.iter().map(|&num| num.abs()).sum();

    Some(magnitude_sum * product_of_signs)
}

fn main() {
    // Example usage:
    let numbers = vec![-2, 3, -4, 5];
    match prod_signs(numbers) {
        Some(result) => println!("Result: {}", result),
        None => println!("Empty array"),
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
