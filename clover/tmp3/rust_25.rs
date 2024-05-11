
fn factorize(n: i32) -> Vec<i32> {
    let mut factors = Vec::new();
    let mut number = n.abs();  // Ensure the factorization of absolute value

    // Factor out the number 2
    while number % 2 == 0 {
        factors.push(2);
        number /= 2;
    }

    // Factor out odd numbers starting from 3
    let mut i = 3;
    while i <= (number as f64).sqrt() as i32 {
        while number % i == 0 {
            factors.push(i);
            number /= i;
        }
        i += 2; // Increment by 2, since there's no need to check even numbers again
    }

    // If the remaining number is a prime number greater than 2, push it to factors
    if number > 2 {
        factors.push(number);
    }

    factors
}

fn main() {
    // Example usage:
    let num = 60;
    println!("The prime factors of {} are: {:?}", num, factorize(num));
}
`
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
