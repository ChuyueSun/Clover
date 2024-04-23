fn main() {}

/*
Return maximum element in the list.

*/

fn maximum(nmbs: Vec<i32>) -> i32 {
    return *nmbs.iter().max().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum() {
        assert!(maximum(vec![1, 2, 3]) == 3);
        assert!(maximum(vec![5, 3, -5, 2, -3, 3, 9, 0, 124, 1, -10]) == 124);
    }
}
