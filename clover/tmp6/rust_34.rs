
fn unique(mut nmbs: Vec<i32>) -> Vec<i32> {
    nmbs.sort_unstable();
    nmbs.dedup();
    nmbs
}

fn main() {
    // Example usage:
    let numbers = vec![3, 6, 2, 3, 2, 6, 7];
    let unique_numbers = unique(numbers);
    println!("{:?}", unique_numbers);
}

#[cfg(test)]
mod tests {
    use super::*;

  #[test]
    fn test_unique() {
        assert!(unique(vec![5, 3, 5, 2, 3, 3, 9, 0, 123]) == vec![0, 2, 3, 5, 9, 123]);
    }


}
