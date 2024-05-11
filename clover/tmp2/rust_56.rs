
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
            }
            _ => (),
        }
    }
    counter == 0
}

fn main() {
    assert_eq!(correct_bracketing("<<>>"), true);
    assert_eq!(correct_bracketing("><"), false);
    assert_eq!(correct_bracketing("<><>"), true);
    assert_eq!(correct_bracketing("<<>"), false);
    println!("All tests passed!");
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
