
fn will_it_fly(q: Vec<i32>, w: i32) -> bool {
    let sum: i32 = q.iter().sum();
    sum <= w && q == q.iter().rev().cloned().collect::<Vec<i32>>()
}

fn main() {
    // Example usage:
    let obj1 = vec![1, 2, 3, 2, 1];
    let max_weight1 = 10;
    println!("Object 1 will fly: {}", will_it_fly(obj1, max_weight1)); // true

    let obj2 = vec![1, 2, 3, 4, 5];
    let max_weight2 = 15;
    println!("Object 2 will fly: {}", will_it_fly(obj2, max_weight2)); // false
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
