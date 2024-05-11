
fn x_or_y(n: i32, x: i32, y: i32) -> i32 {
    if is_prime(n) {
        x
    } else {
        y
    }
}

fn is_prime(num: i32) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=((num as f64).sqrt() as i32) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    // Test cases
    println!("x_or_y(5, 10, 20) = {}", x_or_y(5, 10, 20)); // Should output x i.e., 10
    println!("x_or_y(4, 10, 20) = {}", x_or_y(4, 10, 20)); // Should output y i.e., 20
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x_or_y() {
        assert_eq!(x_or_y(7, 34, 12), 34);
        assert_eq!(x_or_y(15, 8, 5), 5);
        assert_eq!(x_or_y(3, 33, 5212), 33);
        assert_eq!(x_or_y(1259, 3, 52), 3);
        assert_eq!(x_or_y(7919, -1, 12), -1);
        assert_eq!(x_or_y(3609, 1245, 583), 583);
        assert_eq!(x_or_y(91, 56, 129), 129);
        assert_eq!(x_or_y(6, 34, 1234), 1234);
        assert_eq!(x_or_y(1, 2, 0), 0);
        assert_eq!(x_or_y(2, 2, 0), 2);
    }

}
