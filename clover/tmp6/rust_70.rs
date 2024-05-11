
fn strange_sort_list(lst: Vec<i32>) -> Vec<i32> {
    let mut sorted = Vec::new();
    let mut working_lst = lst.clone();
    
    while !working_lst.is_empty() {
        // Push the minimum value
        if let Some(&min) = working_lst.iter().min() {
            sorted.push(min);
            working_lst.retain(|&x| x != min);
        }
        
        // Push the maximum value if still elements left
        if !working_lst.is_empty() {
            if let Some(&max) = working_lst.iter().max() {
                sorted.push(max);
                working_lst.retain(|&x| x != max);
            }
        }
    }
    
    sorted
}

fn main() {
    // Example usage:
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
