
fn intersection(interval1: Vec<i32>, interval2: Vec<i32>) -> String {
    // Determines if a number is prime
    fn is_prime(n: i32) -> bool {
        if n <= 1 {
            return false;
        }
        for i in 2..=((n as f64).sqrt() as i32) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    // Find the intersection of the two intervals
    let start = interval1[0].max(interval2[0]);
    let end = interval1[1].min(interval2[1]);

    // Check if the intervals intersect and find the length of intersection
    if start <= end {
        let length = end - start + 1; // +1 because intervals are closed
        if is_prime(length) {
            "YES".to_string()
        } else {
            "NO".to_string()
        }
    } else {
        "NO".to_string()
    }
}

fn main() {
    // Example usage
    println!("{}", intersection(vec![1, 3], vec![2, 4])); // NO
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