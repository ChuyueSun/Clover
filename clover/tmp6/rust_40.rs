
fn triples_sum_to_zero(nmbs: Vec<i32>) -> bool {
    let len = nmbs.len();
    for i in 0..len {
        for j in i + 1..len {
            for k in j + 1..len {
                if nmbs[i] + nmbs[j] + nmbs[k] == 0 {
                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    // Example usage:
    let numbers = vec![1, -3, 2, 3, 0, -1];
    println!("{}", triples_sum_to_zero(numbers)); // Should print true
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_triples_sum_to_zero() {
        assert!(triples_sum_to_zero(vec![1, 3, 5, 0]) == false);
        assert!(triples_sum_to_zero(vec![1, 3, 5, -1]) == false);
        assert!(triples_sum_to_zero(vec![1, 3, -2, 1]) == true);
        assert!(triples_sum_to_zero(vec![1, 2, 3, 7]) == false);
        assert!(triples_sum_to_zero(vec![1, 2, 5, 7]) == false);
        assert!(triples_sum_to_zero(vec![2, 4, -5, 3, 9, 7]) == true);
        assert!(triples_sum_to_zero(vec![1]) == false);
        assert!(triples_sum_to_zero(vec![1, 3, 5, -100]) == false);
        assert!(triples_sum_to_zero(vec![100, 3, 5, -100]) == false);
    }

}
