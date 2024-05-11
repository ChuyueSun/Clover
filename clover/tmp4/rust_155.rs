
fn even_odd_count(num: i32) -> (i32, i32) {
    let mut even_count = 0;
    let mut odd_count = 0;
    let mut number = num.abs();  // We only need the absolute value to count even/odd digits

    while number > 0 {
        let digit = number % 10;
        if digit % 2 == 0 {
            even_count += 1;
        } else {
            odd_count += 1;
        }
        number /= 10;
    }

    (even_count, odd_count)
}

fn main() {
    let num = 123456;
    let (evens, odds) = even_odd_count(num);
    println!("Even digits: {}, Odd digits: {}", evens, odds);
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
