
fn make_a_pile(n: i32) -> Vec<i32> {
    let mut stones = vec![n];
    let mut current_stones = n;
    for _ in 1..n {
        if current_stones % 2 == 0 {
            current_stones += 2;
        } else {
            current_stones += 1;
        }
        stones.push(current_stones);
    }
    stones
}

fn main() {
    // Example usage:
    let n = 3;
    let pile = make_a_pile(n);
    println!("{:?}", pile); // Should print: [3, 5, 7]
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
