
fn exchange(lst1: Vec<i32>, lst2: Vec<i32>) -> String {
    let mut evens_in_lst1 = lst1.iter().filter(|&&x| x % 2 == 0).count();
    let odds_in_lst1 = lst1.len() - evens_in_lst1;
    let evens_in_lst2 = lst2.iter().filter(|&&x| x % 2 == 0).count();

    if evens_in_lst2 >= odds_in_lst1 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

fn main() {
    // Example usage:
    let lst1 = vec![1, 3, 5];
    let lst2 = vec![2, 4, 6, 8];
    println!("{}", exchange(lst1, lst2)); // Should print "YES"
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
