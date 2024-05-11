
fn strange_sort_list(lst: Vec<i32>) -> Vec<i32> {
    if lst.is_empty() {
        return vec![];
    }
    
    let mut sorted_lst = lst.clone();
    sorted_lst.sort_unstable();
    let mut result = Vec::with_capacity(sorted_lst.len());

    let mut left = 0;
    let mut right = sorted_lst.len() - 1;

    while left <= right {
        result.push(sorted_lst[left]);
        if left != right {
            result.push(sorted_lst[right]);
        }
        left += 1;
        if left <= right {
            right -= 1;
        }
    }
    result
}

fn main() {
    let vec = vec![5, 3, 1, 2, 4];
    let sorted = strange_sort_list(vec);
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
