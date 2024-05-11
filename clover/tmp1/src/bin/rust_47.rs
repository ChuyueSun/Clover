
fn median(mut l: Vec<i32>) -> f32 {
    l.sort_unstable(); // Sort the list
    let length = l.len(); // Get the length of the list

    // Check if the list is empty
    if length == 0 {
        return f32::NAN; // Return NaN if the list is empty
    }

    // Check if the length is even or odd to calculate median
    if length % 2 == 0 {
        // For even length, average the two middle elements
        let mid = length / 2;
        (l[mid - 1] as f32 + l[mid] as f32) / 2.0
    } else {
        // For odd length, return the middle element
        l[length / 2] as f32
    }
}

fn main() {
    // Example usage:
    let numbers = vec![13, 7, 3, 10, 6, 1, 12];
    println!("The median is: {}", median(numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median() {
        assert!(median(vec![3, 1, 2, 4, 5]) == 3.0);
        assert!(median(vec![-10, 4, 6, 1000, 10, 20]) == 8.0);
        assert!(median(vec![5]) == 5.0);
        assert!(median(vec![6, 5]) == 5.5);
        assert!(median(vec![8, 1, 3, 9, 9, 2, 7]) == 7.0);
    }

}
