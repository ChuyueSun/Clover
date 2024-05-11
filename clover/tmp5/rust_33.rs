
fn sort_third(mut l: Vec<i32>) -> Vec<i32> {
    let mut values_to_sort: Vec<i32> = l.iter().enumerate()
        .filter_map(|(i, &number)| if i % 3 == 0 { Some(number) } else { None })
        .collect();

    values_to_sort.sort_unstable();

    for (i, value) in l.iter_mut().enumerate().filter(|&(i, _)| i % 3 == 0) {
        *value = values_to_sort.remove(0);
    }

    l
}

fn main() {
    // Example usage:
    let l = vec![5, 1, 2, 8, 6, 3, 4, 7, 9];
    let sorted = sort_third(l);
    println!("{:?}", sorted);
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
