
fn correct_bracketing(bkts: &str) -> bool {
    let mut counter = 0;

    for b in bkts.chars() {
        match b {
            '<' => counter += 1,
            '>' => {
                if counter == 0 {
                    return false;
                }
                counter -= 1;
            },
            _ => (),
        }
    }

    counter == 0
}

fn main() {
    let brackets = "<<><>>";
    println!("Is the bracketing correct? {}", correct_bracketing(brackets));
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
