
fn even_odd_count(num: i32) -> (i32, i32) {
    let mut num = num.abs(); // Work with the absolute value to handle negative numbers
    let mut evens = 0;
    let mut odds = 0;

    while num > 0 {
        let digit = num % 10;
        if digit % 2 == 0 {
            evens += 1;
        } else {
            odds += 1;
        }
        num /= 10;
    }

    (evens, odds)
}

fn main() {
    // Example usage:
    let number = 123456789;
    let (even_count, odd_count) = even_odd_count(number);
    println!("Even count: {}, Odd count: {}", even_count, odd_count);
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
