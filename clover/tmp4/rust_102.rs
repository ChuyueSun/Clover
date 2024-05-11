
fn choose_num(x: i32, y: i32) -> i32 {
    for num in (x..=y).rev() {
        if num % 2 == 0 {
            return num;
        }
    }
    -1
}

fn main() {
    println!("{}", choose_num(3, 9)); // Should print the largest even number between 3 and 9 (inclusive), which is 8.
    println!("{}", choose_num(10, 10)); // Should print 10 as it's both even and inside the inclusive range.
    println!("{}", choose_num(13, 13)); // Should print -1 as there is no even number in this range.
    println!("{}", choose_num(21, 28)); // Should print 28 as it's the largest even number in this inclusive range.
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
