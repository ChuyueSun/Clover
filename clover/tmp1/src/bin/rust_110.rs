
fn exchange(lst1: Vec<i32>, lst2: Vec<i32>) -> &'static str {
    // Check if lst1 already contains only even numbers
    if lst1.iter().all(|&n| n % 2 == 0) {
        return "YES";
    }

    // Count even and odd numbers in both lists
    let (evens_lst1, odds_lst1) = lst1.iter().fold((0, 0), |(even, odd), &num| {
        if num % 2 == 0 {
            (even + 1, odd)
        } else {
            (even, odd + 1)
        }
    });
    
    let evens_lst2 = lst2.iter().filter(|&&n| n % 2 == 0).count();

    // If there are sufficient even numbers in lst2 to replace odd numbers in lst1, it's possible
    if odds_lst1 <= evens_lst2 {
        "YES"
    } else {
        "NO"
    }
}

fn main() {
    println!("{}", exchange(vec![1, 3, 5], vec![2, 4, 6])); // Should return "YES"
    println!("{}", exchange(vec![1, 3, 5], vec![2, 5, 7])); // Should return "YES"
    println!("{}", exchange(vec![2, 4, 6], vec![1, 3, 5])); // Should return "YES"
    println!("{}", exchange(vec![1, 3, 5], vec![1, 3, 5])); // Should return "NO"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exchange() {
        assert!(exchange(vec![1, 2, 3, 4], vec![1, 2, 3, 4]) == "YES");
        assert!(exchange(vec![1, 2, 3, 4], vec![1, 5, 3, 4]) == "NO");
        assert!(exchange(vec![1, 2, 3, 4], vec![2, 1, 4, 3]) == "YES");
        assert!(exchange(vec![5, 7, 3], vec![2, 6, 4]) == "YES");
        assert!(exchange(vec![5, 7, 3], vec![2, 6, 3]) == "NO");
        assert!(exchange(vec![3, 2, 6, 1, 8, 9], vec![3, 5, 5, 1, 1, 1]) == "NO");
        assert!(exchange(vec![100, 200], vec![200, 200]) == "YES");
    }

}
