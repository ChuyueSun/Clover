
fn correct_bracketing(bkts: &str) -> bool {
    let mut counter = 0;

    for ch in bkts.chars() {
        match ch {
            '<' => counter += 1,
            '>' => {
                if counter == 0 {
                    return false;
                }
                counter -= 1;
            }
            _ => {}
        }
    }

    counter == 0
}

fn main() {
    // Example usage:
    println!("{}", correct_bracketing("<<>>")); // Should print true
    println!("{}", correct_bracketing("<><")); // Should print false
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
