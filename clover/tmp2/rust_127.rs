
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

fn intersection(interval1: (i32, i32), interval2: (i32, i32)) -> &'static str {
    let start = interval1.0.max(interval2.0);
    let end = interval1.1.min(interval2.1);

    if start > end {
        "NO"
    } else if is_prime(end - start + 1) {
        "YES"
    } else {
        "NO"
    }
}

fn main() {
    // Example usage:
    // Intersection is (2,3), which has a length of 2, but 2 is not a prime number.
    println!("{}", intersection((1, 3), (2, 4))); // Outputs: "NO"
    // Intersection is (3,5), which has a length of 3, and 3 is a prime number.
    println!("{}", intersection((1, 5), (3, 7))); // Outputs: "YES"
    // No intersection.
    println!("{}", intersection((1, 2), (3, 4))); // Outputs: "NO"
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
