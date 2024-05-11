
fn prod_signs(arr: Vec<i32>) -> Option<i32> {
    if arr.is_empty() {
        return None;
    }

    let mut magnitude_sum = 0;
    let mut sign_product = 1;
    
    for num in arr {
        magnitude_sum += num.abs();
        sign_product *= match num {
            0 => 0,
            _ => if num > 0 { 1 } else { -1 },
        };
    }
    
    if sign_product == 0 {
        Some(0)
    } else {
        Some(magnitude_sum * sign_product)
    }
}

fn main() {
    // Example usage:
    let array = vec![-1, -2, 3, 0];
    match prod_signs(array) {
        Some(result) => println!("Product of signs: {}", result),
        None => println!("Array was empty"),
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
