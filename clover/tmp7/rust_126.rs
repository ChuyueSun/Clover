
fn is_sorted(lst: Vec<i32>) -> bool {
    let mut prev_val = None;
    let mut counts = std::collections::HashMap::new();

    for &val in &lst {
        if let Some(prev) = prev_val {
            if val < prev {
                return false;
            }
        }
        *counts.entry(val).or_insert(0) += 1;
        if counts[&val] > 1 {
            return false;
        }
        prev_val = Some(val);
    }
    true
}

fn main() {
    // You can test the function here
    let nums = vec![1, 2, 3, 4, 5];
    println!("Is sorted: {}", is_sorted(nums));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_sorted() {
        assert_eq!(is_sorted(vec![5]), true);
        assert_eq!(is_sorted(vec![1, 2, 3, 4, 5]), true);
        assert_eq!(is_sorted(vec![1, 3, 2, 4, 5]), false);
        assert_eq!(is_sorted(vec![1, 2, 3, 4, 5, 6]), true);
        assert_eq!(is_sorted(vec![1, 2, 3, 4, 5, 6, 7]), true);
        assert_eq!(is_sorted(vec![1, 3, 2, 4, 5, 6, 7]), false);
        assert_eq!(is_sorted(vec![]), true);
        assert_eq!(is_sorted(vec![1]), true);
        assert_eq!(is_sorted(vec![3, 2, 1]), false);
        assert_eq!(is_sorted(vec![1, 2, 2, 2, 3, 4]), false);
        assert_eq!(is_sorted(vec![1, 2, 3, 3, 3, 4]), false);
        assert_eq!(is_sorted(vec![1, 2, 2, 3, 3, 4]), true);
        assert_eq!(is_sorted(vec![1, 2, 3, 4]), true);
    }


}
