
fn main() {
    // Test the function with a few examples
    println!("special_factorial(1) = {}", special_factorial(1)); // should be 1
    println!("special_factorial(2) = {}", special_factorial(2)); // should be 2
    println!("special_factorial(3) = {}", special_factorial(3)); // should be 12
    println!("special_factorial(4) = {}", special_factorial(4)); // should be 288
    println!("special_factorial(5) = {}", special_factorial(5)); // should be 34560
}

fn special_factorial(n: i32) -> i64 {
    // Factorial as a separate function to reuse for each n, n-1, n-2, ...
    fn factorial(num: i32) -> i64 {
        (1..=num as i64).product()
    }

    // Compute the Brazilian factorial by calling the factorial function for each i in the range
    let mut result = 1;
    for i in 1..=n {
        result *= factorial(i);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_special_factorial() {
        assert_eq!(special_factorial(4), 288);
        assert_eq!(special_factorial(5), 34560);
        assert_eq!(special_factorial(7), 125411328000);
        assert_eq!(special_factorial(1), 1);
    }

}
