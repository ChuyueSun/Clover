
fn add_even_odd(lst: Vec<i32>) -> i32 {
    lst.iter()
        .enumerate()
        .filter(|(index, &value)| index % 2 != 0 && value % 2 == 0)
        .map(|(_, value)| value)
        .sum()
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
    println!("Sum of even elements at odd indices: {}", add_even_odd(numbers));
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