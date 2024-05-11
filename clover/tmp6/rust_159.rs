
fn eat(number: i32, need: i32, remaining: i32) -> [i32; 2] {
    let eatable = i32::min(need, remaining);
    [number + eatable, remaining - eatable]
}

fn main() {
    // Example usage
    println!("{:?}", eat(10, 5, 3)); // Should print [13, 0]
    println!("{:?}", eat(5, 10, 10)); // Should print [15, 0]
    println!("{:?}", eat(0, 8, 20)); // Should print [8, 12]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eat() {
        assert_eq!(eat(5, 6, 10), vec![11, 4]);
        assert_eq!(eat(4, 8, 9), vec![12, 1]);
        assert_eq!(eat(1, 10, 10), vec![11, 0]);
        assert_eq!(eat(2, 11, 5), vec![7, 0]);
        assert_eq!(eat(4, 5, 7), vec![9, 2]);
        assert_eq!(eat(4, 5, 1), vec![5, 0]);
    }

}
