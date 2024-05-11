
fn intersection(interval1: Vec<i32>, interval2: Vec<i32>) -> &'static str {
    // Define a helper function to check if a number is prime
    fn is_prime(num: i32) -> bool {
        if num <= 1 {
            return false;
        }
        for i in 2..=(num as f64).sqrt() as i32 {
            if num % i == 0 {
                return false;
            }
        }
        true
    }

    // Calculate the start and end points of the intersection
    let start = interval1[0].max(interval2[0]);
    let end = interval1[1].min(interval2[1]);

    // Check if there is an intersection
    if start > end {
        "NO"
    } else {
        // Calculate the length of the intersection
        let length = end - start + 1;
        // Determine if the length of the intersection is a prime number
        if is_prime(length) {
            "YES"
        } else {
            "NO"
        }
    }
}

// Example usage:
// let result = intersection(vec![1, 3], vec![2, 4]);
// assert_eq!(result, "NO");

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
