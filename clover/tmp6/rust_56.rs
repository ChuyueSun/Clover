
fn correct_bracketing(bkts: &str) -> bool {
    let mut counter = 0;
    for c in bkts.chars() {
        match c {
            '<' => counter += 1,
            '>' => {
                if counter == 0 {
                    return false;
                }
                counter -= 1;
            }
            _ => return false, // Non-bracket characters are not allowed based on the docstring
        }
    }
    counter == 0
}

fn main() {
    // Example usage:
    println!("{}", correct_bracketing("<><>")); // true
    println!("{}", correct_bracketing("<<>>")); // true
    println!("{}", correct_bracketing("><")); // false
    println!("{}", correct_bracketing("<<>")); // false
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
