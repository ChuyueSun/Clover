
fn sort_even(nmbs: Vec<i32>) -> Vec<i32> {
    let mut even_indices: Vec<(usize, &i32)> = nmbs.iter().enumerate().filter(|&(i, _)| i % 2 == 0).collect();
    even_indices.sort_unstable_by_key(|&(_, &val)| val);

    let mut result = nmbs.clone();
    for (sorted_index, &(original_index, _)) in even_indices.iter().enumerate() {
        result[original_index] = *even_indices[sorted_index].1;
    }

    result
}

fn main() {
    let l = vec![5, 3, 2, 2, 1, 9, 8, 13, 6];
    let l_sorted = sort_even(l);
    println!("{:?}", l_sorted);
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
