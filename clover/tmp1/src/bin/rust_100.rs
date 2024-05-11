
fn make_a_pile(n: i32) -> Vec<i32> {
    let mut stones = n;
    let mut pile = vec![];

    for _ in 0..n {
        pile.push(stones);
        stones += if stones % 2 == 0 { 2 } else { 1 };
    }

    pile
}

fn main() {
    // Example usage:
    let n = 5;
    let pile = make_a_pile(n);
    println!("{:?}", pile);  // This should print [5, 7, 9, 11, 13]
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
