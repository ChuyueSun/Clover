
fn will_it_fly(q: Vec<i32>, w: i32) -> bool {
    let sum: i32 = q.iter().sum();
    sum <= w && q == q.iter().rev().cloned().collect::<Vec<_>>()
}

fn main() {
    // Test cases
    println!("{}", will_it_fly(vec![1, 2, 3, 2, 1], 10)); // Should print true
    println!("{}", will_it_fly(vec![1, 2, 3, 4, 5], 15)); // Should print false
    println!("{}", will_it_fly(vec![10, 1, 10], 30));     // Should print true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_will_it_fly() {
        assert!(will_it_fly(vec![3, 2, 3], 9) == true);
        assert!(will_it_fly(vec![1, 2], 5) == false);
        assert!(will_it_fly(vec![3], 5) == true);
        assert!(will_it_fly(vec![3, 2, 3], 1) == false);
        assert!(will_it_fly(vec![1, 2, 3], 6) == false);
        assert!(will_it_fly(vec![5], 5) == true);
    }

}
