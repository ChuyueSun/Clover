
fn starts_or_ends_one(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    (2 * 10i32.pow((n - 1) as u32)) - 1
}

fn main() {
    let count = starts_or_ends_one(3);
    println!("There are {} numbers of 3-digit that start or end with 1", count);
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
