
fn is_sorted(lst: Vec<i32>) -> bool {
    if lst.is_empty() {
        return true;
    }
    
    let mut prev = lst[0];
    let mut count = 1;
    
    for &item in lst.iter().skip(1) {
        if item == prev {
            count += 1;
            if count > 1 {
                return false;
            }
        } else if item < prev {
            return false;
        } else {
            count = 0;
        }
        prev = item;
    }

    true
}

fn main() {
    // Test cases:
    println!("{}", is_sorted(vec![1, 2, 3, 4])); // true
    println!("{}", is_sorted(vec![1, 1, 2, 3])); // false
    println!("{}", is_sorted(vec![4, 1, 2, 3])); // false
    println!("{}", is_sorted(vec![1, 1]));       // false
    println!("{}", is_sorted(vec![]));           // true
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
