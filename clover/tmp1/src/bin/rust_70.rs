
fn strange_sort_list(lst: Vec<i32>) -> Vec<i32> {
    let mut sorted_lst = lst.clone();
    sorted_lst.sort();
    
    let mut result = Vec::with_capacity(sorted_lst.len());
    while !sorted_lst.is_empty() {
        if let Some(min) = sorted_lst.first().cloned() {
            result.push(min);
            sorted_lst.remove(0);
        }
        
        if let Some(max) = sorted_lst.last().cloned() {
            result.push(max);
            sorted_lst.pop();
        }
    }
    
    result
}

fn main() {
    let lst = vec![4, 2, 6, 3, 1, 5];
    let sorted = strange_sort_list(lst);
    println!("{:?}", sorted);
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
