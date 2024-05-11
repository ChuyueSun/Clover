
fn sort_array_1(mut arr: Vec<i32>) -> Vec<i32> {
    fn count_ones(x: &i32) -> i32 {
        x.count_ones() as i32
    }

    arr.sort_unstable_by(|a, b| {
        let ones_a = count_ones(a);
        let ones_b = count_ones(b);
        match ones_a.cmp(&ones_b) {
            std::cmp::Ordering::Equal => a.cmp(b),
            other => other,
        }
    });
    arr
}

fn main() {
    let mut arr = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    arr = sort_array_1(arr);
    println!("{:?}", arr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_array_1() {
        assert!(sort_array_1(vec![1, 5, 2, 3, 4]) == vec![1, 2, 4, 3, 5]);
        assert!(sort_array_1(vec![-2, -3, -4, -5, -6]) == vec![-4, -2, -6, -5, -3]);
        assert!(sort_array_1(vec![1, 0, 2, 3, 4]) == vec![0, 1, 2, 4, 3]);
        assert!(sort_array_1(vec![]) == vec![]);
        assert!(
            sort_array_1(vec![2, 5, 77, 4, 5, 3, 5, 7, 2, 3, 4])
                == vec![2, 2, 4, 4, 3, 3, 5, 5, 5, 7, 77]
        );
        assert!(sort_array_1(vec![3, 6, 44, 12, 32, 5]) == vec![32, 3, 5, 6, 12, 44]);
        assert!(sort_array_1(vec![2, 4, 8, 16, 32]) == vec![2, 4, 8, 16, 32]);
        assert!(sort_array_1(vec![2, 4, 8, 16, 32]) == vec![2, 4, 8, 16, 32]);
    }

}
