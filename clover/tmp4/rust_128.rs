
fn prod_signs(arr: Vec<i32>) -> Option<i32> {
    if arr.is_empty() {
        None
    } else {
        let mut sign = 1;
        for &num in &arr {
            match num {
                n if n < 0 => sign *= -1,
                0 => return Some(0),
                _ => {}
            }
        }
        let sum_of_magnitudes: i32 = arr.iter().map(|&num| num.abs()).sum();
        Some(sum_of_magnitudes * sign)
    }
}

fn main() {
    // Example usage
    let numbers = vec![-1, 2, -3, 4, 0];
    println!("{:?}", prod_signs(numbers)); // Output: Some(0)
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
