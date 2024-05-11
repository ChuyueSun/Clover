
fn rounded_avg(n: i32, m: i32) -> i32 {
    if n > m {
        -1
    } else {
        let count = m - n + 1;
        let sum: i32 = (n..=m).sum();
        let avg = (sum as f32 / count as f32).round() as i32;
        format!("{:b}", avg).parse().unwrap()
    }
}

fn main() {
    // Example usage:
    println!("{}", rounded_avg(5, 9)); // Should print the average of 5, 6, 7, 8, 9 in binary
    println!("{}", rounded_avg(10, 10)); // Should print the average of 10 in binary
    println!("{}", rounded_avg(20, 15)); // Should print -1
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
