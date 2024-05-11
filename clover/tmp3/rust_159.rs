
fn eat(number: i32, need: i32, remaining: i32) -> [i32; 2] {
    let carrots_eaten = if need <= remaining {
        need
    } else {
        remaining
    };
    let total_eaten = number + carrots_eaten;
    let carrots_left = if need <= remaining {
        remaining - need
    } else {
        0
    };

    [total_eaten, carrots_left]
}

fn main() {
    // You can test the function here
    let result = eat(300, 500, 400); // Example input
    println!("{:?}", result); // Expected output: [800, 0]
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
