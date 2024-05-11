
fn unique_digits(x: Vec<i32>) -> Vec<i32> {
    x.into_iter()
        .filter(|&num| !num.to_string().contains('0')
            && !num.to_string().contains('2')
            && !num.to_string().contains('4')
            && !num.to_string().contains('6')
            && !num.to_string().contains('8'))
        .sorted()
        .collect()
}

fn main() {
    let numbers = vec![123, 555, 322, 49, 37];
    let result = unique_digits(numbers);
    println!("{:?}", result);
}
```

**Note**: You will need to add `use itertools::Itertools;` to your imports if you want to use `.sorted()`. Otherwise, you can replace `.sorted()` with the standard library's `.sort_unstable()` but to use it, you need to collect items into a Vec first and then call `sort_unstable()` as shown:

```rust
fn unique_digits(mut x: Vec<i32>) -> Vec<i32> {
    x.retain(|&num| !num.to_string().contains('0')
        && !num.to_string().contains('2')
        && !num.to_string().contains('4')
        && !num.to_string().contains('6')
        && !num.to_string().contains('8'));
    x.sort_unstable();
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_digits() {
        assert!(unique_digits(vec![15, 33, 1422, 1]) == vec![1, 15, 33]);
        assert!(unique_digits(vec![152, 323, 1422, 10]) == vec![]);
        assert!(unique_digits(vec![12345, 2033, 111, 151]) == vec![111, 151]);
        assert!(unique_digits(vec![135, 103, 31]) == vec![31, 135]);
    }

}
