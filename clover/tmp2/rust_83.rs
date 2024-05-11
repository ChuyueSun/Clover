
fn starts_one_ends(n: i32) -> i32 {
    if n < 1 {
        0
    } else if n == 1 {
        1
    } else {
        // For n>1, there are 2 * 9^(n-1) minus 1 because the numbers that start and end in 1 are counted twice.
        2 * 9_i32.pow((n - 1) as u32) - 1
    }
}

fn main() {
    let n = 3;
    println!("The count of {}-digit numbers starting or ending with 1 is: {}", n, starts_one_ends(n));
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
