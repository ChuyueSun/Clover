
fn sort_third(mut l: Vec<i32>) -> Vec<i32> {
    let mut to_sort: Vec<(usize, &i32)> = l.iter().enumerate().filter(|&(i, _)| i % 3 == 0).collect();
    to_sort.sort_by(|a, b| a.1.cmp(b.1));

    let mut sorted_indices = to_sort.into_iter().map(|(i, _)| i).peekable();

    for i in 0..l.len() {
        if let Some(&next_sorted_index) = sorted_indices.peek() {
            if i == next_sorted_index {
                if let Some((sorted_index, _)) = sorted_indices.next() {
                    l.swap(i, sorted_index);
                }
            }
        }
    }

    l
}

fn main() {
    let l = vec![5, 2, 8, 1, 4, 9, 3, 7, 6];
    let l_sorted = sort_third(l);
    println!("{:?}", l_sorted);
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
