
fn factorial(n: i32) -> i64 {
    let mut result = 1;
    for i in 1..=n {
        result *= i as i64;
    }
    result
}

fn special_factorial(n: i32) -> i64 {
    let mut result = 1;
    for i in 1..=n {
        result *= factorial(i);
    }
    result
}

fn main() {
    println!("Special factorial of 5 is {}", special_factorial(5));
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
