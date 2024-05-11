
fn generate_integers(a: i32, b: i32) -> Vec<i32> {
    let mut result = Vec::new();
    for n in a..=b {
        if n % 2 == 0 {
            result.push(n);
        }
    }
    result
}

fn main() {
    let a = 5;
    let b = 15;
    let even_integers = generate_integers(a, b);
    println!("{:?}", even_integers);
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
