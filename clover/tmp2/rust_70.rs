
fn strange_sort_list(mut lst: Vec<i32>) -> Vec<i32> {
    let mut sorted_list = Vec::new();
    lst.sort_unstable(); // Sort the list inplace to help find min and max quickly
    let mut left = 0; // Start of the list
    let mut right = lst.len(); // End of the list

    while left < right {
        sorted_list.push(lst[left]);        // Push the minimum element
        left += 1;                          // Move the start rightwards
        if left < right {                   // If there are elements remaining
            right -= 1;                     // Move the end leftwards
            sorted_list.push(lst[right]);   // Push the maximum element
        }
    }
    sorted_list
}

fn main() {
    let numbers = vec![10, 5, 2, 3, 7, 9];
    let strange_sorted_numbers = strange_sort_list(numbers);
    println!("{:?}", strange_sorted_numbers);
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
