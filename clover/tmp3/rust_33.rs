
fn sort_third(l: Vec<i32>) -> Vec<i32> {
    let mut sort_indices: Vec<usize> = l.iter().enumerate().filter_map(|(i, _)| {
        if i % 3 == 0 { Some(i) } else { None }
    }).collect();

    let mut values_to_sort: Vec<i32> = sort_indices.iter().map(|&i| l[i]).collect();
    values_to_sort.sort_unstable();
    
    let mut l_prime = l.clone();
    for (&index, &value) in sort_indices.iter().zip(values_to_sort.iter()) {
        l_prime[index] = value;
    }
    
    l_prime
}

fn main() {
    let list = vec![17, 4, 3, 1, 6, 9, 8, 2, 5];
    let sorted_thirds = sort_third(list);
    println!("{:?}", sorted_thirds);
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_sort_third() {
        let mut l = vec![1, 2, 3];
        assert_eq!(sort_third(l), vec![1, 2, 3]);
        l = vec![5, 3, -5, 2, -3, 3, 9, 0, 123, 1, -10];
        assert_eq!(sort_third(l), vec![5, 3, -5, 1, -3, 3, 2, 0, 123, 9, -10]);
        l = vec![5, 8, -12, 4, 23, 2, 3, 11, 12, -10];
        assert_eq!(sort_third(l), vec![5, 8, -12, -10, 23, 2, 3, 11, 12, 4]);
        l = vec![5, 6, 3, 4, 8, 9, 2];
        assert_eq!(sort_third(l), vec![5, 6, 3, 2, 8, 9, 4]);
        l = vec![5, 8, 3, 4, 6, 9, 2];
        assert_eq!(sort_third(l), vec![5, 8, 3, 2, 6, 9, 4]);
        l = vec![5, 6, 9, 4, 8, 3, 2];
        assert_eq!(sort_third(l), vec![5, 6, 9, 2, 8, 3, 4]);
        l = vec![5, 6, 3, 4, 8, 9, 2, 1];
        assert_eq!(sort_third(l), vec![5, 6, 3, 2, 8, 9, 4, 1]);
    }

}
