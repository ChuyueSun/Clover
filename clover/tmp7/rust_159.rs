
fn eat(number: i32, need: i32, remaining: i32) -> [i32; 2] {
    let to_eat = if need <= remaining { need } else { remaining };
    [number + to_eat, remaining - to_eat]
}

fn main() {
    // Example usage:
    let number = 5; // Number of carrots already eaten 
    let need = 8; // Number of carrots you need to eat
    let remaining = 10; // Number of remaining carrots in stock

    let result = eat(number, need, remaining);
    println!("{:?}", result); // Should print: [13, 2]
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
