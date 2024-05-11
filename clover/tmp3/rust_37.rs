
fn sort_even(mut nmbs: Vec<i32>) -> Vec<i32> {
    let mut even_indices_values = nmbs.iter().enumerate().filter_map(|(i, &x)| {
        if i % 2 == 0 {
            Some(x)
        } else {
            None
        }
    }).collect::<Vec<i32>>();
    
    even_indices_values.sort_unstable();
    
    for (i, pair) in even_indices_values.into_iter().enumerate() {
        nmbs[i * 2] = pair;
    }

    nmbs
}

fn main() {
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    let sorted_numbers = sort_even(numbers);
    println!("{:?}", sorted_numbers);
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
