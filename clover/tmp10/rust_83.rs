
fn starts_one_ends(n: i32) -> i32 {
    // For n = 1, the number is trivially starting and ending with 1.
    if n == 1 {
        return 1;
    }

    // For n-digit numbers, there are 9 possibilities (1-9) for each non-leading/non-trailing digit.
    let inner_possibilities = 9_i32.pow((n - 2) as u32);

    // There are two cases for a n-digit number to start or end with 1:
    // 1. The number starts with 1 and can end with any digit [0-9] which gives us 10 possibilities.
    // 2. The number ends with 1 but does not start with 1, which gives us 9 (excluding the starting 1) possibilities.
    // However, there is an overlap, i.e., the number '11...1' where the number starts and ends with 1.
    // This overlap is counted twice, so we need to subtract it.
    let overlap = 1;

    // Calculate the totals for both cases and adjust for the overlap.
    let total = (inner_possibilities * 10) + (inner_possibilities * 9) - overlap;

    total
}

fn main() {
    // Example usages of the `starts_one_ends` function.
    println!("The count of 3-digit numbers starting or ending with 1 is: {}", starts_one_ends(3));
    println!("The count of 5-digit numbers starting or ending with 1 is: {}", starts_one_ends(5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starts_one_ends() {
        assert!(starts_one_ends(1) == 1);
        assert!(starts_one_ends(2) == 18);
        assert!(starts_one_ends(3) == 180);
        assert!(starts_one_ends(4) == 1800);
        assert!(starts_one_ends(5) == 18000);
    }

}
