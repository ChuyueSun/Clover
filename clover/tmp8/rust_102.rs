
fn choose_num(x: i32, y: i32) -> i32 {
    if x > y {
        return -1; // early return when range is not valid
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
    println!("Biggest even number between 3 and 9 is: {}", choose_num(3, 9));
    println!("Biggest even number between 10 and 10 is: {}", choose_num(10, 10));
    println!("Biggest even number between 13 and 13 is: {}", choose_num(13, 13));
    println!("Biggest even number between 19 and 11 is: {}", choose_num(19, 11));
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
