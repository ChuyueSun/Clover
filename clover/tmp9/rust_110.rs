
fn exchange(lst1: Vec<i32>, lst2: Vec<i32>) -> &'static str {
    // Count odd numbers in lst1 as they need to be exchanged.
    let odd_count_lst1 = lst1.iter().filter(|&&x| x % 2 != 0).count();

    // Count even numbers in lst2 which can be exchanged with odd in lst1.
    let even_count_lst2 = lst2.iter().filter(|&&x| x % 2 == 0).count();

    // Check if there are enough even numbers in lst2 to exchange with the odd numbers in lst1.
    if odd_count_lst1 <= even_count_lst2 {
        "YES"
    } else {
        "NO"
    }
}

fn main() {
    // Example test case
    let lst1 = vec![1, 3, 5];
    let lst2 = vec![2, 4, 6];
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
