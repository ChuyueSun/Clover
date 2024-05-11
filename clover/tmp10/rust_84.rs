
fn solve(n: i32) -> String {
    format!("{:b}", n.to_string().chars().map(|d| d.to_digit(10).unwrap()).sum::<u32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(solve(0), "0");
        assert_eq!(solve(12345), "1110");
        assert_eq!(solve(9999), "111011100");
        assert_eq!(solve(10000), "1");
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(solve(1), "1");
        assert_eq!(solve(10), "1");
        assert_eq!(solve(100), "1");
        assert_eq!(solve(911), "10110");
        assert_eq!(solve(8210), "10100");
    }
}

fn main() {
    println!("The sum of the digits of 12345 in binary: {}", solve(12345));
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
