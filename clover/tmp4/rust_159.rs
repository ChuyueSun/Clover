
fn eat(number: i32, need: i32, remaining: i32) -> [i32; 2] {
    let to_eat = need.min(remaining);
    [number + to_eat, remaining - to_eat]
}

fn main() {
    // Example usage:
    // You have eaten 5 carrots, you need to eat 8 more, and there's 10 remaining.
    let result = eat(5, 8, 10);
    println!("Total eaten: {}, Carrots left: {}", result[0], result[1]);
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
