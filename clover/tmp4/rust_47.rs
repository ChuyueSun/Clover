
fn median(mut l: Vec<i32>) -> Option<f32> {
    if l.is_empty() {
        return None;
    }

    l.sort_unstable();
    let mid = l.len() / 2;

    if l.len() % 2 == 0 {
        Some((l[mid - 1] as f32 + l[mid] as f32) / 2.0)
    } else {
        Some(l[mid] as f32)
    }
}

fn main() {
    let data1 = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let data2 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let data3: Vec<i32> = vec![];

    println!("Median of data1 is {:?}", median(data1));
    println!("Median of data2 is {:?}", median(data2));
    println!("Median of data3 is {:?}", median(data3));
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
