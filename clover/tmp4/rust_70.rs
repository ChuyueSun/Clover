
fn strange_sort_list(lst: Vec<i32>) -> Vec<i32> {
    let mut sorted_list = vec![];
    let mut lst_clone = lst.clone();
    lst_clone.sort_unstable();

    while !lst_clone.is_empty() {
        // take the minimum value and append it to sorted_list
        sorted_list.push(lst_clone.remove(0));
        if !lst_clone.is_empty() {
            // take the maximum value and append it to sorted_list
            sorted_list.push(lst_clone.pop().unwrap());
        }
    }

    sorted_list
}

fn main() {
    let list_of_integers = vec![5, 3, 2, 8, 1, 4];
    let sorted_list = strange_sort_list(list_of_integers);
    println!("{:?}", sorted_list);
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
