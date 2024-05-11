
/// Returns the maximum element in the given list of numbers.
///
/// # Arguments
/// * `nmbs` - A vector of i32 integers.
///
/// # Returns
/// * `i32` - The maximum value in the nmbs vector. If the vector is empty, returns `i32::MIN`.
///
/// # Examples
/// ```
/// let numbers = vec![1, 2, 3, 4, 5];
/// assert_eq!(maximum(numbers), 5);
/// ```
fn maximum(nmbs: Vec<i32>) -> i32 {
    *nmbs.iter().max().unwrap_or(&i32::MIN)
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    println!("The maximum number is: {}", maximum(numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_maximum() {
        assert!(maximum(vec![1, 2, 3]) == 3);
        assert!(maximum(vec![5, 3, -5, 2, -3, 3, 9, 0, 124, 1, -10]) == 124);
    }


}
