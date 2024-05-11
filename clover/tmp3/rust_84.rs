
fn solve(n: i32) -> String {
    let sum_of_digits = n.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>();
    format!("{:b}", sum_of_digits)
}

fn main() {
    // Example usage:
    let n = 1234;
    println!("The sum of digits of {} in binary is {}", n, solve(n));
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
