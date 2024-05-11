
fn will_it_fly(q: Vec<i32>, w: i32) -> bool {
    let sum: i32 = q.iter().sum();
    let is_palindrome = q.iter().eq(q.iter().rev());
    is_palindrome && sum <= w
}

fn main() {
    // Example use:
    let q = vec![1, 2, 3, 2, 1];
    let w = 10;
    println!("Will it fly? {}", will_it_fly(q, w));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_will_it_fly() {
        assert!(will_it_fly(vec![3, 2, 3], 9) == true);
        assert!(will_it_fly(vec![1, 2], 5) == false);
        assert!(will_it_fly(vec![3], 5) == true);
        assert!(will_it_fly(vec![3, 2, 3], 1) == false);
        assert!(will_it_fly(vec![1, 2, 3], 6) == false);
        assert!(will_it_fly(vec![5], 5) == true);
    }

}
