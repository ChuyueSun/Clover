
fn incr_list(l: Vec<i32>) -> Vec<i32> {
    l.into_iter().map(|x| x + 1).collect()
}

fn main() {
    let list = vec![1, 2, 3, 4, 5];
    let incremented_list = incr_list(list);
    println!("{:?}", incremented_list);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incr_list() {
        assert!(incr_list(vec![]) == vec![]);
        assert!(incr_list(vec![3, 2, 1]) == [4, 3, 2]);
        assert!(incr_list(vec![5, 2, 5, 2, 3, 3, 9, 0, 123]) == [6, 3, 6, 3, 4, 4, 10, 1, 124]);
    }

}
