
fn even_odd_count(num: i32) -> (i32, i32) {
    let mut even_count = 0;
    let mut odd_count = 0;
    let mut n = num.abs(); // working with absolute value to handle negative numbers

    while n > 0 {
        if n % 2 == 0 {
            even_count += 1;
        } else {
            odd_count += 1;
        }
        n /= 10; // move to the next digit
    }

    (even_count, odd_count)
}

fn main() {
    // Example usage:
    let num = 123456;
    let (even_count, odd_count) = even_odd_count(num);
    println!("In the number {}: Even digits = {}, Odd digits = {}", num, even_count, odd_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_odd() {
        assert_eq!(even_odd_count(7), vec![0, 1]);
        assert_eq!(even_odd_count(-78), vec![1, 1]);
        assert_eq!(even_odd_count(3452), vec![2, 2]);
        assert_eq!(even_odd_count(346211), vec![3, 3]);
        assert_eq!(even_odd_count(-345821), vec![3, 3]);
        assert_eq!(even_odd_count(-2), vec![1, 0]);
        assert_eq!(even_odd_count(-45347), vec![2, 3]);
        assert_eq!(even_odd_count(0), vec![1, 0]);
    }

}
