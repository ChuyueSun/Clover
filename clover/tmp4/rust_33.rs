
fn sort_third(l: Vec<i32>) -> Vec<i32> {
    let mut sorted_thirds: Vec<i32> = l.iter()
        .enumerate()
        .filter(|&(i, _)| i % 3 == 0)
        .map(|(_, &val)| val)
        .collect();

    sorted_thirds.sort_unstable();

    let mut sorted_idx = 0;
    l.iter()
        .enumerate()
        .map(|(i, &val)| {
            if i % 3 == 0 {
                sorted_idx += 1;
                sorted_thirds[sorted_idx - 1]
            } else {
                val
            }
        })
        .collect()
}

fn main() {
    let l = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let sorted_l = sort_third(l);
    println!("{:?}", sorted_l);
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
