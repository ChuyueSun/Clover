
fn solve(n: i32) -> String {
    let sum_of_digits = n.to_string().chars()
                         .map(|c| c.to_digit(10).unwrap())
                         .sum::<u32>();
    format!("{:b}", sum_of_digits)
}

fn main() {
    // You can test the function with a number here
    let result = solve(123);
    println!("The binary sum of digits is: {}", result);
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
