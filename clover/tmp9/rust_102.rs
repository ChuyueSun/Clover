
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
    println!("{}", choose_num(1, 4)); // Should print 4
    println!("{}", choose_num(7, 12)); // Should print 12
    println!("{}", choose_num(8, 5)); // Should print -1
    println!("{}", choose_num(9, 9)); // Should print -1
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
