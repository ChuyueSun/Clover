
fn solve(n: i32) -> String {
    let sum = sum_of_digits(n);
    format!("{:b}", sum)
}

fn sum_of_digits(mut n: i32) -> i32 {
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

fn main() {
    // Example usage:
    let result = solve(12345); // For N=12345, the sum of digits is 15, which is "1111" in binary
    println!("The sum of digits in binary is: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert!(solve(1000) == "1");
        assert!(solve(150) == "110");
        assert!(solve(147) == "1100");
        assert!(solve(333) == "1001");
        assert!(solve(963) == "10010");
    }

}
