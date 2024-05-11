
fn sort_third(mut l: Vec<i32>) -> Vec<i32> {
    let mut sorted_thirds: Vec<i32> = l.iter()
        .enumerate()
        .filter(|(i, _)| i % 3 == 0)
        .map(|(_, &val)| val)
        .collect();
    
    sorted_thirds.sort_unstable();
    
    for (i, val) in l.iter_mut().enumerate() {
        if i % 3 == 0 {
            *val = sorted_thirds.remove(0);
        }
    }
    
    l
}

fn main() {
    let l = vec![3, 2, 1, 6, 5, 4, 9, 8, 7, 12, 11, 10];
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
