
fn special_factorial(n: i32) -> i64 {
    if n <= 0 {
        return 1;
    }

    let mut result: i64 = 1;
    for i in 1..=n as i64 {
        result *= factorial(i);
    }
    result
}

fn factorial(n: i64) -> i64 {
    (1..=n).product()
}

fn main() {
    let n = 5;
    println!("The Brazilian factorial of {} is {}", n, special_factorial(n));
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
