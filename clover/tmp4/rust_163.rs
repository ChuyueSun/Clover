
fn generate_integers(a: i32, b: i32) -> Vec<i32> {
    (a..=b).filter(|x| x % 2 == 0).collect()
}

fn main() {
    // Example usage:
    let even_digits = generate_integers(3, 10);
    println!("Even numbers between 3 and 10: {:?}", even_digits);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_integers() {
        assert_eq!(generate_integers(2, 10), vec![2, 4, 6, 8]);
        assert_eq!(generate_integers(10, 2), vec![2, 4, 6, 8]);
        assert_eq!(generate_integers(132, 2), vec![2, 4, 6, 8]);
        assert_eq!(generate_integers(17, 89), vec![]);
    }

}
