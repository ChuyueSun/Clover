
fn solve(n: i32) -> String {
    let sum_of_digits = n.to_string()
                          .chars()
                          .map(|c| c.to_digit(10).unwrap() as i32)
                          .sum::<i32>();
    format!("{:b}", sum_of_digits)
}

fn main() {
    // Example usage:
    let result = solve(123); // For n = 123, sum of digits is 6 (1+2+3), binary of 6 is "110".
    println!("The sum of the digits in binary: {}", result);
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
