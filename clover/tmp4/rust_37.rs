
fn sort_even(nmbs: Vec<i32>) -> Vec<i32> {
    let mut even_indices: Vec<(usize, i32)> = nmbs
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(i, &v)| (i, v))
        .collect();
    
    even_indices.sort_by(|a, b| a.1.cmp(&b.1));
    
    let mut result = nmbs.clone();
    for (index, value) in even_indices.iter() {
        result[*index] = *value;
    }
    
    result
}

fn main() {
    let nmbs = vec![4, 1, 3, 9, 2, 5, 11, 7, 8];
    let sorted = sort_even(nmbs);
    println!("{:?}", sorted);
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_sort_even() {
        assert_eq!(sort_even(vec![1, 2, 3]), vec![1, 2, 3]);
        assert_eq!(
            sort_even(vec![5, 3, -5, 2, -3, 3, 9, 0, 123, 1, -10]),
            vec![-10, 3, -5, 2, -3, 3, 5, 0, 9, 1, 123]
        );
        assert_eq!(
            sort_even(vec![5, 8, -12, 4, 23, 2, 3, 11, 12, -10]),
            vec![-12, 8, 3, 4, 5, 2, 12, 11, 23, -10]
        );
    }

}
