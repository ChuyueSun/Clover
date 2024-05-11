
fn correct_bracketing(bkts: &str) -> bool {
    let mut count = 0;
    for c in bkts.chars() {
        match c {
            '<' => count += 1,
            '>' => if count == 0 { return false } else { count -= 1 },
            _ => {}
        }
    }
    count == 0
}

fn main() {
    // Example usage:
    println!("{}", correct_bracketing("<<>>")); // returns true
    println!("{}", correct_bracketing("<><>")); // returns true
    println!("{}", correct_bracketing("><"));   // returns false
    println!("{}", correct_bracketing("<<<>")); // returns false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_bracketing() {
        assert!(correct_bracketing("<>"));
        assert!(correct_bracketing("<<><>>"));
        assert!(correct_bracketing("<><><<><>><>"));
        assert!(correct_bracketing("<><><<<><><>><>><<><><<>>>"));
        assert!(!(correct_bracketing("<<<><>>>>")));
        assert!(!(correct_bracketing("><<>")));
        assert!(!(correct_bracketing("<")));
        assert!(!(correct_bracketing("<<<<")));
        assert!(!(correct_bracketing(">")));
        assert!(!(correct_bracketing("<<>")));
        assert!(!(correct_bracketing("<><><<><>><>><<>")));
        assert!(!(correct_bracketing("<><><<><>><>>><>")));
    }

}
