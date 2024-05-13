
fn rounded_avg(n: i32, m: i32) -> String {
    if n > m {
        return "-1".to_string();
    }
    let sum: i32 = (n..=m).sum();
    println!("{}", sum);
    let count: i32 = m - n + 1;
    println!("{}", sum as f64 / count as f64);
    let average: i32 = (sum as f64 / count as f64).round() as i32;
    println!("{}", average);
    format!("{:b}", average)
}

fn main() {
    // Example usage:
    println!("{}", rounded_avg(2, 4)); // Output for 2, 3, 4 average (rounded): "11" which is binary for 3
    println!("{}", rounded_avg(5, 9)); // Output for 5, 6, 7, 8, 9 average (rounded): "111" which is binary for 7
    println!("{}", rounded_avg(10, 10)); // Output when n equals m: "1010" which is binary for 10
    println!("{}", rounded_avg(7, 4)); // Output when n is greater than m: "-1"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rounded_avg() {
        assert!(rounded_avg(1, 5) == "11");
        assert!(rounded_avg(7, 13) == "1010");
        assert!(rounded_avg(964, 977) == "1111001010");
        assert!(rounded_avg(996, 997) == "1111100100");
        assert!(rounded_avg(560, 851) == "1011000001");
        assert!(rounded_avg(185, 546) == "101101101");
        assert!(rounded_avg(362, 496) == "110101101");
        assert!(rounded_avg(350, 902) == "1001110010");
        assert!(rounded_avg(197, 233) == "11010111");
        assert!(rounded_avg(7, 5) == "-1");
        assert!(rounded_avg(5, 1) == "-1");
        assert!(rounded_avg(5, 5) == "101");
    }

}
