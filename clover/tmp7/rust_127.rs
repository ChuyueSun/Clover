
fn intersection(interval1: Vec<i32>, interval2: Vec<i32>) -> &'static str {
    let left = interval1[0].max(interval2[0]);
    let right = interval1[1].min(interval2[1]);
    if left > right {
        return "NO";
    }
    let length = right - left + 1;
    if is_prime(length) {
        "YES"
    } else {
        "NO"
    }
}

fn is_prime(number: i32) -> bool {
    if number <= 1 {
        return false;
    }
    for i in 2..=((number as f64).sqrt() as i32) {
        if number % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    // Example usage:
    let interval1 = vec![1, 3];
    let interval2 = vec![2, 4];
    println!("{}", intersection(interval1, interval2)); // Output: "NO"
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
