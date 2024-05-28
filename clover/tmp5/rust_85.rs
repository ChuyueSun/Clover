
fn add_even_odd(lst: Vec<i32>) -> i32 {
    lst.iter()
        .enumerate()
        .filter(|&(i, &x)| i % 2 == 1 && x % 2 == 0)
        .map(|(_, &x)| x)
        .sum()
}

fn main() {
    let numbers = vec![1, 4, 2, 3, 5, 8, 6];
    println!("Sum: {}", add_even_odd(numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_even_odd() {
        assert!(add_even_odd(vec![4, 88]) == 88);
        assert!(add_even_odd(vec![4, 5, 6, 7, 2, 122]) == 122);
        assert!(add_even_odd(vec![4, 0, 6, 7]) == 0);
        assert!(add_even_odd(vec![4, 4, 6, 8]) == 12);
    }


}