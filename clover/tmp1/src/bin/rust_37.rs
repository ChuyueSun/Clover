
fn sort_even(mut nmbs: Vec<i32>) -> Vec<i32> {
    let mut even_indices_elements: Vec<(usize, i32)> = nmbs
        .iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 0)
        .map(|(i, &val)| (i, val))
        .collect();

    even_indices_elements.sort_by_key(|&(_, val)| val);

    for (i, val) in even_indices_elements {
        nmbs[i] = val;
    }

    nmbs
}

fn main() {
    let nmbs = vec![5, 3, 2, 4, 1, 6];
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
