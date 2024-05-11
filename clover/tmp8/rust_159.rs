
fn eat(number: i32, need: i32, remaining: i32) -> [i32; 2] {
    let mut total_eaten = number;
    let mut left_over = remaining;

    let to_eat = if need >= remaining { remaining } else { need };
    total_eaten += to_eat;
    left_over -= to_eat;

    [total_eaten, left_over]
}

fn main() {
    // Example usage:
    println!("{:?}", eat(5, 10, 6)); // [11, 0]
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
