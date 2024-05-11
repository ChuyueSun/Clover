
fn special_factorial(n: i32) -> i128 {
    let mut result = 1i128;

    for i in 1..=n {
        result *= factorial(i);
    }

    result
}

fn factorial(num: i32) -> i128 {
    let mut fact = 1i128;
    for i in 1..=num {
        fact *= i as i128;
    }
    fact
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
