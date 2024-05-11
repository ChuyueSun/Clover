
fn intersection(interval1: Vec<i32>, interval2: Vec<i32>) -> String {
    // Utility function to check for primality
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

    // Calculate the intersection
    let start = interval1[0].max(interval2[0]);
    let end = interval1[1].min(interval2[1]);

    // Check if there is intersection
    if start > end {
        return "NO".to_string();
    }

    // Calculate the length of the intersection
    let length = end - start + 1; 

    // Check if the length of intersection is prime
    if is_prime(length) {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

// Example usage
fn main() {
    assert_eq!(intersection(vec![1, 3], vec![2, 4]), "NO");
    assert_eq!(intersection(vec![1, 10], vec![5, 15]), "YES"); // Intersection length is 6, not a prime
    assert_eq!(intersection(vec![10, 20], vec![0, 5]), "NO");  // No intersection
    // ... You can add additional test cases here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        assert_eq!(intersection(vec![1, 2], vec![2, 3]), "NO");
        assert_eq!(intersection(vec![-1, 1], vec![0, 4]), "NO");
        assert_eq!(intersection(vec![-3, -1], vec![-5, 5]), "YES");
        assert_eq!(intersection(vec![-2, 2], vec![-4, 0]), "YES");
        assert_eq!(intersection(vec![-11, 2], vec![-1, -1]), "NO");
        assert_eq!(intersection(vec![1, 2], vec![3, 5]), "NO");
        assert_eq!(intersection(vec![1, 2], vec![1, 2]), "NO");
        assert_eq!(intersection(vec![-2, -2], vec![-3, -2]), "NO");
    }

}
