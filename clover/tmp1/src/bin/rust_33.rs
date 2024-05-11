
fn sort_third(mut l: Vec<i32>) -> Vec<i32> {
    let mut indices_and_values: Vec<(usize, i32)> = l.iter()
        .enumerate()
        .filter(|(i, _)| i % 3 == 0)
        .map(|(i, &v)| (i, v))
        .collect();

    indices_and_values.sort_by_key(|&(_, v)| v);
    println!("{:?}", indices_and_values);

    for (i, v) in indices_and_values {
        l[i] = v;
    }

    l
}

fn main() {
    let l = vec![8, 5, 10, 2, 6, 1, 3, 9, 4];
    let sorted = sort_third(l);
    println!("??????");
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
