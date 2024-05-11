
fn add_elements(arr: Vec<i32>, k: i32) -> i32 {
    arr.iter()
       .take(k as usize)
       .filter(|&&x| x >= -99 && x <= 99)
       .sum()
}

fn main() {
    // Example usage:
    let numbers = vec![10, 123, 56, 101, 99];
    let k = 3;
    println!("The sum is: {}", add_elements(numbers, k));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_elements() {
        assert_eq!(add_elements(vec![1, -2, -3, 41, 57, 76, 87, 88, 99], 3), -4);
        assert_eq!(add_elements(vec![111, 121, 3, 4000, 5, 6], 2), 0);
        assert_eq!(add_elements(vec![11, 21, 3, 90, 5, 6, 7, 8, 9], 4), 125);
        assert_eq!(add_elements(vec![111, 21, 3, 4000, 5, 6, 7, 8, 9], 4), 24);
        assert_eq!(add_elements(vec![1], 1), 1);
    }

}
