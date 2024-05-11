
fn add_elements(arr: Vec<i32>, k: i32) -> i32 {
    arr.iter()
    .take(k as usize)
    .filter(|&&x| x.abs() < 100)
    .sum()
}

fn main() {
    // Example usage:
    let arr = vec![10, 5, 100, 3, -1, 98, 105];
    let k = 6;
    let result = add_elements(arr, k);
    println!("The sum is: {}", result);
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
