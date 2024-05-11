
fn median(mut l: Vec<i32>) -> Option<f32> {
    if l.is_empty() {
        None
    } else {
        l.sort_unstable();
        let mid = l.len() / 2;
        if l.len() % 2 == 0 {
            Some((l[mid - 1] as f32 + l[mid] as f32) / 2.0)
        } else {
            Some(l[mid] as f32)
        }
    }
}

fn main() {
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    println!("The median is: {:?}", median(numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median() {
        assert!(median(vec![3, 1, 2, 4, 5]) == 3.0);
        assert!(median(vec![-10, 4, 6, 1000, 10, 20]) == 8.0);
        assert!(median(vec![5]) == 5.0);
        assert!(median(vec![6, 5]) == 5.5);
        assert!(median(vec![8, 1, 3, 9, 9, 2, 7]) == 7.0);
    }

}
