
fn strange_sort_list(lst: Vec<i32>) -> Vec<i32> {
    let mut sorted = lst.clone();
    sorted.sort();
    
    let mut result = Vec::new();
    while !sorted.is_empty() {
        if let Some(min) = sorted.pop() {
            result.push(min);
        }
        
        if let Some(max) = sorted.pop() {
            result.insert(result.len() - 1, max);
        }
    }
    result
}

fn main() {
    let lst = vec![5, 2, 9, 1, 5, 6];
    let sorted_lst = strange_sort_list(lst);
    println!("{:?}", sorted_lst);
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_strange_sort_list() {
        assert!(strange_sort_list(vec![1, 2, 3, 4]) == vec![1, 4, 2, 3]);
        assert!(strange_sort_list(vec![5, 6, 7, 8, 9]) == vec![5, 9, 6, 8, 7]);
        assert!(strange_sort_list(vec![1, 2, 3, 4, 5]) == vec![1, 5, 2, 4, 3]);
        assert!(strange_sort_list(vec![5, 6, 7, 8, 9, 1]) == vec![1, 9, 5, 8, 6, 7]);
        assert!(strange_sort_list(vec![5, 5, 5, 5]) == vec![5, 5, 5, 5]);
        assert!(strange_sort_list(vec![]) == vec![]);
        assert!(strange_sort_list(vec![1, 2, 3, 4, 5, 6, 7, 8]) == vec![1, 8, 2, 7, 3, 6, 4, 5]);
        assert!(
            strange_sort_list(vec![0, 2, 2, 2, 5, 5, -5, -5]) == vec![-5, 5, -5, 5, 0, 2, 2, 2]
        );
        assert!(strange_sort_list(vec![111111]) == vec![111111]);
    }

}
