
fn eat(number: i32, need: i32, remaining: i32) -> [i32; 2] {
    let mut total_eaten = number;
    let mut carrots_left = remaining;

    let can_eat = std::cmp::min(need, remaining);
    total_eaten += can_eat;
    carrots_left -= can_eat;

    [total_eaten, carrots_left]
}

fn main() {
    // Example usage:
    let number_eaten = 5;      // Number of carrots already eaten.
    let need_eat = 10;         // Number of additional carrots needed.
    let remaining_carrots = 8; // Number of remaining carrots available.

    // Perform the operation
    let result = eat(number_eaten, need_eat, remaining_carrots);
    
    // Print the result
    println!("Total carrots eaten: {}, Carrots remaining: {}", result[0], result[1]);
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
