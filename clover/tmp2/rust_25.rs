
fn factorize(n: i32) -> Vec<i32> {
    let mut factors = Vec::new();
    let mut num = n;
    let mut divisor = 2;

    while num > 1 {
        while num % divisor == 0 {
            factors.push(divisor);
            num /= divisor;
        }
        divisor += 1;
    }
    
    factors
}

fn main() {
    // Example usage
    let number = 60;
    println!("Prime factors of {}: {:?}", number, factorize(number));
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_factorize() {
        assert_eq!(factorize(2), vec![2]);
        assert_eq!(factorize(4), vec![2, 2]);
        assert_eq!(factorize(8), vec![2, 2, 2]);
        assert_eq!(factorize(3 * 19), vec![3, 19]);
        assert_eq!(factorize(3 * 19 * 3 * 19), vec![3, 3, 19, 19]);
        assert_eq!(
            factorize(3 * 19 * 3 * 19 * 3 * 19),
            vec![3, 3, 3, 19, 19, 19]
        );
        assert_eq!(factorize(3 * 19 * 19 * 19), vec![3, 19, 19, 19]);
        assert_eq!(factorize(3 * 2 * 3), vec![2, 3, 3]);
    }

}
