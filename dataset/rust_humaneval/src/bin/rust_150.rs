fn main() {}

/*
A simple program which should return the value of x if n is
    a prime number and should return the value of y otherwise.

*/

fn x_or_y(n: i32, x: i32, y: i32) -> i32 {
    let mut isp = true;
    if n < 2 {
        isp = false;
    }
    for i in 2..=n / 2 {
        if n % i == 0 {
            isp = false;
        }
    }
    if isp {
        return x;
    }
    return y;
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
