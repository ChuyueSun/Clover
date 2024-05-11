
fn correct_bracketing(bkts: &str) -> bool {
    let mut balance = 0;
    for b in bkts.chars() {
        match b {
            '<' => balance += 1,
            '>' => {
                if balance == 0 {
                    return false;
                }
                balance -= 1;
            }
            _ => return false, // invalid characters
        }
    }
    balance == 0
}

fn main() {
    // Example usage:
    println!("{}", correct_bracketing("<<>>")); // returns true
    println!("{}", correct_bracketing("><")); // returns false
    println!("{}", correct_bracketing("<>")); // returns true
    println!("{}", correct_bracketing("<<><>")); // returns true
    println!("{}", correct_bracketing(">>>")); // returns false
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
