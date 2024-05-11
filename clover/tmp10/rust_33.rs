
fn sort_third(l: Vec<i32>) -> Vec<i32> {
    // Separate the elements at indices divisible by three from the others
    let (mut thirds, mut others): (Vec<i32>, Vec<i32>) = (vec![], vec![]);
    for (i, &value) in l.iter().enumerate() {
        if i % 3 == 0 {
            thirds.push(value);
        } else {
            others.push(value);
        }
    }

    // Sort only the elements at indices divisible by three
    thirds.sort_unstable();

    // Merge them back into the vector, placing sorted elements in the right positions
    let mut sorted_thirds_iter = thirds.into_iter();
    let mut combined = Vec::with_capacity(l.len());
    for i in 0..l.len() {
        if i % 3 == 0 {
            if let Some(sorted_value) = sorted_thirds_iter.next() {
                combined.push(sorted_value);
            }
        } else {
            combined.push(others.remove(0));
        }
    }

    combined
}

fn main() {
    let input = vec![5, 1, 3, 6, 2, 9];
    let sorted = sort_third(input);
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
