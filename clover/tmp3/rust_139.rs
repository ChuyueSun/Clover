
fn special_factorial(n: i32) -> i64 {
    if n <= 1 {
        return 1;
    }

    let mut result:i64 = 1;
    for i in 1..=n {
        result *= factorial(i) as i64;
    }
    result
}

fn factorial(num: i32) -> i32 {
    (1..=num).product()
}

fn main() {
    // Example usage:
    let result = special_factorial(5);
    println!("The Brazilian factorial of 5 is {}", result);
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
