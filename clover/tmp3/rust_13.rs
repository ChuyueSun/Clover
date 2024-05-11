
fn greatest_common_divisor(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a.abs()
}

fn main() {
    // Example usage:
    let a = 54;
    let b = 24;
    let gcd = greatest_common_divisor(a, b);
    println!("The greatest common divisor of {} and {} is {}", a, b, gcd);
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_greatest_common_divisor() {
        assert!(greatest_common_divisor(3, 7) == 1);
        assert!(greatest_common_divisor(10, 15) == 5);
        assert!(greatest_common_divisor(49, 14) == 7);
        assert!(greatest_common_divisor(144, 60) == 12);
    }

}
