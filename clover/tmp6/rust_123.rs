
fn get_odd_collatz(n: i32) -> Vec<i32> {
    let mut v = Vec::new();
    let mut number = n;
    
    while number != 1 {
        if number % 2 != 0 {
            v.push(number);
        }
        number = if number % 2 == 0 {
            number / 2
        } else {
            3 * number + 1
        };
    }
    
    if n != 0 {
        v.push(1);
    }
    
    v.sort_unstable();
    v
}

fn main() {
    // Example usage:
    // To see the output, you can un-comment the println statement below and run the program
    let result = get_odd_collatz(13);
    // println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

   #[test]
    fn test_get_odd_collatz() {
        assert_eq!(get_odd_collatz(14), vec![1, 5, 7, 11, 13, 17]);
        assert_eq!(get_odd_collatz(5), vec![1, 5]);
        assert_eq!(get_odd_collatz(12), vec![1, 3, 5]);
        assert_eq!(get_odd_collatz(1), vec![1]);
    }

}
