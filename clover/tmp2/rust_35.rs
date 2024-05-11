
fn maximum(nmbs: Vec<i32>) -> i32 {
    *nmbs.iter().max().expect("Cannot find maximum of an empty list")
}

fn main() {
    let numbers = vec![4, 1, 8, 3, 5, 2, 6, 7];
    let max = maximum(numbers);
    println!("The maximum number is {}", max);
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
