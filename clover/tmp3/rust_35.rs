
fn maximum(nmbs: Vec<i32>) -> i32 {
    *nmbs.iter().max().unwrap()
}

fn main() {
    // Example usage:
    let numbers = vec![1, 5, 2, 7, 3];
    println!("The maximum number is: {}", maximum(numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_maximum() {
        assert!(maximum(vec![1, 2, 3]) == 3);
        assert!(maximum(vec![5, 3, -5, 2, -3, 3, 9, 0, 124, 1, -10]) == 124);
    }


}
