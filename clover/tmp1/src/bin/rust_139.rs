
fn special_factorial(n: i32) -> i128 {
    fn factorial(k: i32) -> i128 {
        (1..=k).fold(1, |acc, x| acc * (x as i128))
    }

    (1..=n).rev().fold(1, |acc, x| acc * factorial(x))
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
