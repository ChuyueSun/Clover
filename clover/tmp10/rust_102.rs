
fn choose_num(x: i32, y: i32) -> i32 {
    if x > y {
        return -1;
    }
    
    for num in (x..=y).rev() {
        if num % 2 == 0 {
            return num;
        }
    }

    -1
}

fn main() {
    // Example usage:
    println!("{}", choose_num(1, 10)); // Should print 10
    println!("{}", choose_num(9, 15)); // Should print 14
    println!("{}", choose_num(23, 23)); // Should print -1 since 23 is odd
    println!("{}", choose_num(24, 24)); // Should print 24
    println!("{}", choose_num(25, 30)); // Should print 30
    println!("{}", choose_num(17, 17)); // Should print -1 since there are no even numbers in this range
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
