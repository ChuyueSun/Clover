
fn can_arrange(arr: Vec<i32>) -> i32 {
    for (i, &item) in arr.iter().enumerate().skip(1) {
        if item < arr[i - 1] {
            return i as i32;
        }
    }
    -1
}

fn main() {
    // Test cases
    let example1 = vec![1, 2, 3, 4, 5]; // should return -1 as all elements are in increasing order
    let example2 = vec![5, 4, 3, 2, 1]; // should return 1 as 4 < 5
    let example3 = vec![10, 20, 15, 30, 40]; // should return 2 as 15 < 20

    println!("can_arrange([1, 2, 3, 4, 5]) = {}", can_arrange(example1));
    println!("can_arrange([5, 4, 3, 2, 1]) = {}", can_arrange(example2));
    println!("can_arrange([10, 20, 15, 30, 40]) = {}", can_arrange(example3));
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_can_arrange() {
        assert_eq!(can_arrange(vec![1, 2, 4, 3, 5]), 3);
        assert_eq!(can_arrange(vec![1, 2, 4, 5]), -1);
        assert_eq!(can_arrange(vec![1, 4, 2, 5, 6, 7, 8, 9, 10]), 2);
        assert_eq!(can_arrange(vec![4, 8, 5, 7, 3]), 4);
        assert_eq!(can_arrange(vec![]), -1);
    }


}
