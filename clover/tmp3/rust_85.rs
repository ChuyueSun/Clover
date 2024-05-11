
/// Given a non-empty list of integers `lst`, add the even elements that are at odd indices.
///
/// # Arguments
///
/// * `lst` - A vector of `i32` integers.
///
/// # Returns
///
/// * The sum of the even elements at odd indices.
///
/// # Examples
///
/// ```
/// let numbers = vec![1, 2, 3, 4, 5, 6];
/// let result = add_even_odd(numbers);
/// assert_eq!(result, 6);
/// ```
fn add_even_odd(lst: Vec<i32>) -> i32 {
    lst.iter()
       .enumerate()
       .filter(|(idx, &val)| idx % 2 != 0 && val % 2 == 0)
       .map(|(_, val)| val)
       .sum()
}

fn main() {
    // Example usage:
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let result = add_even_odd(numbers);
    println!("The sum is: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_even_odd() {
        assert!(add_even_odd(vec![4, 88]) == 88);
        assert!(add_even_odd(vec![4, 5, 6, 7, 2, 122]) == 122);
        assert!(add_even_odd(vec![4, 0, 6, 7]) == 0);
        assert!(add_even_odd(vec![4, 4, 6, 8]) == 12);
    }


}
