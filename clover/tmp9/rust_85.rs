
fn add_even_odd(lst: Vec<i32>) -> i32 {
    lst.iter()
        .enumerate()
        .filter(|&(i, &x)| i % 2 != 0 && x % 2 == 0)
        .map(|(_, x)| x)
        .sum()
}

fn main() {
    // An example usage of the function `add_even_odd`
    let nums = vec![1, 2, 3, 4, 5, 6];
    println!("The sum is: {}", add_even_odd(nums));
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
