
fn will_it_fly(q: Vec<i32>, w: i32) -> bool {
    q == q.iter().rev().cloned().collect::<Vec<i32>>() && q.iter().sum::<i32>() <= w
}

fn main() {
    // Example usage:
    let q = vec![1, 3, 3, 1];
    let w = 10;
    println!("Will it fly? {}", will_it_fly(q, w)); // Should print "Will it fly? true"
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
