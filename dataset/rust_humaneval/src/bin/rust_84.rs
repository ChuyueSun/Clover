fn main() {}

/*
Given a positive integer N, return the total sum of its digits in binary.

    Variables:
        @N integer
             Constraints: 0 ≤ N ≤ 10000.
    Output:
         a string of binary number

*/

fn solve(n: i32) -> String {
    let sum: i32 = n
        .to_string()
        .chars()
        .into_iter()
        .fold(0, |acc, c| acc + c.to_digit(10).unwrap() as i32);
    return format!("{sum:b}");
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
