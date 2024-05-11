
fn exchange(lst1: Vec<i32>, lst2: Vec<i32>) -> &'static str {
    let mut lst1_odd_count = lst1.iter().filter(|&&x| x % 2 != 0).count();
    let lst2_even_count = lst2.iter().filter(|&&x| x % 2 == 0).count();

    if lst1_odd_count == 0 {
        "YES"
    } else if lst2_even_count >= lst1_odd_count {
        "YES"
    } else {
        "NO"
    }
}

fn main() {
    assert_eq!(exchange(vec![2, 4, 6], vec![1, 3, 5]), "YES");
    assert_eq!(exchange(vec![2, 3, 4], vec![1, 3, 5]), "YES");
    assert_eq!(exchange(vec![2, 3, 4], vec![1, 3]), "NO");
    println!("All test cases passed!");
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
