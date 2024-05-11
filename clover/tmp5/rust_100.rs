
fn make_a_pile(n: i32) -> Vec<i32> {
    let mut stones_per_level = Vec::new();
    let mut stones_in_current_level = n;
    for _ in 0..n {
        stones_per_level.push(stones_in_current_level);
        if stones_in_current_level % 2 == 0 {
            stones_in_current_level += 2;
        } else {
            stones_in_current_level += 1;
        }
    }
    stones_per_level
}

fn main() {
    // Example usage:
    let levels = make_a_pile(5);
    println!("{:?}", levels); // Output should be [5, 6, 7, 8, 9]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_a_pile() {
        assert!(make_a_pile(3) == vec![3, 5, 7]);
        assert!(make_a_pile(4) == vec![4, 6, 8, 10]);
        assert!(make_a_pile(5) == vec![5, 7, 9, 11, 13]);
        assert!(make_a_pile(6) == vec![6, 8, 10, 12, 14, 16]);
        assert!(make_a_pile(8) == vec![8, 10, 12, 14, 16, 18, 20, 22]);
    }

}
