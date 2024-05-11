
fn special_factorial(n: i32) -> i64 {
    if n <= 0 {
        return 0;
    }
    let mut result = 1_i64;
    for i in 1..=n as i64 {
        result *= factorial(i);
    }
    result
}

fn factorial(n: i64) -> i64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    // Test the function with a sample integer
    println!("The Brazilian factorial of 5 is {}", special_factorial(5));
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
