
fn solve(n: i32) -> String {
    n.to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|digit| format!("{:b}", digit))
        .fold(0, |acc, b| acc + binary_string_to_num(&b))
        .to_string()
}

fn binary_string_to_num(binary: &str) -> i32 {
    i32::from_str_radix(binary, 2).unwrap_or(0)
}

fn main() {
    let result = solve(123); // Example usage
    println!("Sum of digits in binary: {}", result);
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
