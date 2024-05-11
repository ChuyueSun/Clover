
fn digits(n: i32) -> i32 {
    let mut product = 1;
    let mut has_odd = false;
    let mut number = n;

    while number > 0 {
        let digit = number % 10;
        if digit % 2 != 0 {
            product *= digit;
            has_odd = true;
        }
        number /= 10;
    }

    if has_odd {
        product
    } else {
        0
    }
}

fn main() {
    println!("{}", digits(1234)); // Output: 3
    println!("{}", digits(2468)); // Output: 0
    println!("{}", digits(13579)); // Output: 945
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        assert_eq!(digits(5), 5);
        assert_eq!(digits(54), 5);
        assert_eq!(digits(120), 1);
        assert_eq!(digits(5014), 5);
        assert_eq!(digits(98765), 315);
        assert_eq!(digits(5576543), 2625);
        assert_eq!(digits(2468), 0);
    }

}
