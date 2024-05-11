
fn correct_bracketing(bkts: &str) -> bool {
    let mut count = 0;
    for c in bkts.chars() {
        match c {
            '<' => count += 1,
            '>' => {
                if count == 0 {
                    return false;
                }
                count -= 1;
            }
            _ => {}
        }
    }

    count == 0
}

fn main() {
    // Example usage:
    println!("{}", correct_bracketing("<<>>")); // Prints true
    println!("{}", correct_bracketing("<><>")); // Prints true
    println!("{}", correct_bracketing("><"));   // Prints false
    println!("{}", correct_bracketing(">>"));   // Prints false
    println!("{}", correct_bracketing("<<><")); // Prints false
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
