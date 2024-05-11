
fn exchange(lst1: Vec<i32>, lst2: Vec<i32>) -> String {
    let has_even_in_lst2 = lst2.iter().any(|&x| x % 2 == 0);
    let odd_count_in_lst1 = lst1.iter().filter(|&&x| x % 2 != 0).count();
    
    if has_even_in_lst2 && odd_count_in_lst1 > 0 {
        "YES".to_string()
    } else if odd_count_in_lst1 == 0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

fn main() {
    // Example usage:
    let lst1 = vec![1, 3, 5];
    let lst2 = vec![2, 4, 6];
    println!("{}", exchange(lst1, lst2)); // Should print "YES"

    let lst1 = vec![1, 3, 5];
    let lst2 = vec![1, 3, 5];
    println!("{}", exchange(lst1, lst2)); // Should print "NO"
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
