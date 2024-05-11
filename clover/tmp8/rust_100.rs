
fn make_a_pile(n: i32) -> Vec<i32> {
    let mut pile = Vec::new();
    let mut current = n;
    for _ in 0..n {
        pile.push(current);
        current = if current % 2 == 0 { current + 2 } else { current + 2 };
    }
    pile
}

fn main() {
    let n = 5;
    let pile = make_a_pile(n);
    for (i, stones) in pile.iter().enumerate() {
        println!("Level {}: {} stones", i + 1, stones);
    }
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
