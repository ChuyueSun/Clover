
fn choose_num(x: i32, y: i32) -> i32 {
    for i in (x..=y).rev() {
        if i % 2 == 0 {
            return i;
        }
    }
    -1
}

fn main() {
    // Example usage
    println!("Biggest even number in range: {}", choose_num(3, 10)); // Should print 10
    println!("Biggest even number in range: {}", choose_num(3, 3));  // Should print -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choose_num() {
        assert!(choose_num(12, 15) == 14);
        assert!(choose_num(13, 12) == -1);
        assert!(choose_num(33, 12354) == 12354);
        assert!(choose_num(6, 29) == 28);
        assert!(choose_num(27, 10) == -1);
        assert!(choose_num(7, 7) == -1);
        assert!(choose_num(546, 546) == 546);
    }

}