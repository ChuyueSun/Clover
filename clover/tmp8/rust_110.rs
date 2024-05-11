
fn exchange(lst1: Vec<i32>, lst2: Vec<i32>) -> &'static str {
    if lst1.iter().all(|&x| x % 2 == 0) {
        "YES"
    } else {
        let lst1_odd_count = lst1.iter().filter(|&x| x % 2 != 0).count();
        let lst2_even_count = lst2.iter().filter(|&x| x % 2 == 0).count();
        if lst2_even_count >= lst1_odd_count {
            "YES"
        } else {
            "NO"
        }
    }
}

fn main() {
    // Example usage
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
