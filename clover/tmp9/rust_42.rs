
/// Increments each element in the given vector by 1 and returns the new vector.
///
/// # Examples
///
/// ```
/// let numbers = vec![1, 2, 3];
/// let incremented_numbers = incr_list(numbers);
/// assert_eq!(incremented_numbers, vec![2, 3, 4]);
/// ```
///
/// # Arguments
///
/// * `l` - A vector of i32 elements to be incremented
///
/// # Returns
///
/// A new vector with each element from the input vector incremented by 1
fn incr_list(l: Vec<i32>) -> Vec<i32> {
    l.into_iter().map(|x| x + 1).collect()
}

fn main() {
    let numbers = vec![1, 2, 3];
    let incremented_numbers = incr_list(numbers);
    println!("{:?}", incremented_numbers);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incr_list() {
        assert!(incr_list(vec![]) == vec![]);
        assert!(incr_list(vec![3, 2, 1]) == [4, 3, 2]);
        assert!(incr_list(vec![5, 2, 5, 2, 3, 3, 9, 0, 123]) == [6, 3, 6, 3, 4, 4, 10, 1, 124]);
    }

}
