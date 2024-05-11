
fn eat(number: i32, need: i32, remaining: i32) -> [i32; 2] {
    if remaining >= need {
        [number + need, remaining - need]
    } else {
        [number + remaining, 0]
    }
}

fn main() {
    // Example usage:
    println!("{:?}", eat(5, 10, 20)); // Should print: [15, 10]
    println!("{:?}", eat(7, 12, 10)); // Should print: [17, 0]
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
