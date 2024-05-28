
fn largest_divisor(n: i32) -> i32 {
    if n <= 1 {
        return 0;
    }
    for i in (2..n).rev() {
        if n % i == 0 {
            return i;
        }
    }
    1
}

fn main() {
    let num = 10;
    println!("The largest divisor of {} is {}", num, largest_divisor(num));
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