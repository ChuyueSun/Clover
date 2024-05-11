
fn sort_even(mut nmbs: Vec<i32>) -> Vec<i32> {
    let mut even_indices: Vec<_> = nmbs.iter().enumerate().filter_map(|(i, &v)| if i % 2 == 0 { Some(v) } else { None }).collect();
    even_indices.sort_unstable();
    for (sorted_val, nmb) in even_indices.into_iter().zip(nmbs.iter_mut().step_by(2)) {
        *nmb = sorted_val;
    }
    nmbs
}

fn main() {
    let nums = vec![5, 3, 2, 4, 1];
    let sorted = sort_even(nums);
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
