fn main() {}

/*
Return sorted unique elements in a list

*/

fn unique(nmbs: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = nmbs.clone();
    res.sort();
    res.dedup();
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique() {
        assert!(unique(vec![5, 3, 5, 2, 3, 3, 9, 0, 123]) == vec![0, 2, 3, 5, 9, 123]);
    }
}
