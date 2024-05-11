
fn eat(number: i32, need: i32, remaining: i32) -> [i32; 2] {
    let can_eat = need.min(remaining);
    let eaten = number + can_eat;
    let left = remaining - can_eat;
    [eaten, left]
}

fn main() {
    // Example usage:
    let number = 5;
    let need = 10;
    let remaining = 7;
    let result = eat(number, need, remaining);
    println!("Eaten: {}, Left: {}", result[0], result[1]);
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
