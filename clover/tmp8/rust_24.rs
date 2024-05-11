
fn largest_divisor(n: i32) -> i32 {
    for i in (1..n).rev() {
        if n % i == 0 {
            return i;
        }
    }
    1 // If no divisor is found, then it's 1 (for n=1 case)
}

fn main() {
    let number = 10;
    println!("The largest divisor of {} is {}", number, largest_divisor(number));
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_largest_divisor() {
        assert!(largest_divisor(3) == 1);
        assert!(largest_divisor(7) == 1);
        assert!(largest_divisor(10) == 5);
        assert!(largest_divisor(100) == 50);
        assert!(largest_divisor(49) == 7);
    }

}
