
fn intersection(interval1: Vec<i32>, interval2: Vec<i32>) -> String {
    // Find the max of the start values and the min of the end values of the intervals
    let start = interval1[0].max(interval2[0]);
    let end = interval1[1].min(interval2[1]);
    
    // Check if the intervals intersect
    if start > end {
        return "NO".to_string();
    }
    
    // Calculate the length of the intersection
    let length = end - start + 1;
    
    // Determine if the length is a prime number
    if is_prime(length) {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

// Helper function to check if a number is prime
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

fn main() {
    // Example usage:
    // Intersection is (2, 3) with length 2, which is a prime number.
    println!("{}", intersection(vec![1, 3], vec![2, 4])); // output: "YES"

    // Intersection is (4, 6) with length 3, which is a prime number.
    println!("{}", intersection(vec![1, 7], vec![4, 6])); // output: "YES"

    // No intersection, so the result is "NO".
    println!("{}", intersection(vec![1, 2], vec![3, 4])); // output: "NO"
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
