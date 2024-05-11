
fn sort_third(mut l: Vec<i32>) -> Vec<i32> {
    let mut values_to_sort: Vec<i32> = l.iter()
        .enumerate()
        .filter(|&(i, _)| i % 3 == 0)
        .map(|(_, &x)| x)
        .collect();
    
    values_to_sort.sort_unstable();
    
    for (i, value) in values_to_sort.into_iter().enumerate() {
        l[i * 3] = value;
    }
    
    l
}

fn main() {
    let l = vec![5, 3, 1, 2, 4, 6, 9, 8, 7]; // Example input
    let l_prime = sort_third(l);
    println!("{:?}", l_prime); // Expected output would be something like [2, 3, 1, 5, 4, 6, 9, 8, 7]
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
