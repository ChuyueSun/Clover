
fn factorize(n: i32) -> Vec<i32> {
    let mut factors = Vec::new();
    let mut num = n;
    let mut factor = 2;

    while factor * factor <= num {
        while num % factor == 0 {
            factors.push(factor);
            num /= factor;
        }
        factor += 1;
    }

    if num > 1 {
        factors.push(num);
    }

    factors
}

fn main() {
    // Example:
    let number = 60;
    println!("{:?}", factorize(number)); // Output: [2, 2, 3, 5]
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
