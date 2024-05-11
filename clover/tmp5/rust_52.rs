
fn below_threshold(l: Vec<i32>, t: i32) -> bool {
    l.iter().all(|&num| num < t)
}

fn main() {
    // Example usage:
    let numbers = vec![1, 2, 3, 4, 5];
    let threshold = 6;
    
    println!("All numbers below threshold? {}", below_threshold(numbers, threshold)); // Should print "All numbers below threshold? true"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_below_threshold() {
        assert!(below_threshold(vec![1, 2, 4, 10], 100));
        assert!(!below_threshold(vec![1, 20, 4, 10], 5));
        assert!(below_threshold(vec![1, 20, 4, 10], 21));
        assert!(below_threshold(vec![1, 20, 4, 10], 22));
        assert!(below_threshold(vec![1, 8, 4, 10], 11));
        assert!(!below_threshold(vec![1, 8, 4, 10], 10));
    }

}
