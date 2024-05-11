
fn special_factorial(n: i32) -> i64 {
    let mut result = 1;
    for i in 1..=n as i64 {
        result *= traditional_factorial(i);
    }
    result
}

fn traditional_factorial(n: i64) -> i64 {
    let mut factorial = 1;
    for i in 1..=n {
        factorial *= i;
    }
    factorial
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
