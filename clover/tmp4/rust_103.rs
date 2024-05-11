
fn rounded_avg(n: i32, m: i32) -> String {
    if n > m {
        return "-1".to_string();
    }
    let total_numbers = m - n + 1;
    let sum: i32 = (n..=m).sum();
    let average = (sum as f64 / total_numbers as f64).round() as i32;
    format!("{:b}", average)
}

fn main() {
    println!("{}", rounded_avg(5, 10)); // For example
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
