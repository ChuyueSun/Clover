
fn fruit_distribution(s: &str, n: i32) -> i32 {
    let fruits: Vec<&str> = s.split_whitespace().collect();
    let (mut apples, mut oranges) = (0, 0);

    for fruit in fruits {
        if fruit.ends_with("apple") {
            let count: i32 = fruit.trim_end_matches("apple").parse().unwrap_or(0);
            apples += count;
        } else if fruit.ends_with("orange") {
            let count: i32 = fruit.trim_end_matches("orange").parse().unwrap_or(0);
            oranges += count;
        }
    }

    n - (apples + oranges)
}

// Example usage (not part of the requested function):
fn main() {
    let s = "10apples 5oranges";
    let n = 20;
    let mangoes = fruit_distribution(s, n);
    println!("Number of mangoes: {}", mangoes); // Should print "Number of mangoes: 5"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fruit_distribution() {
        assert!(fruit_distribution("5 apples and 6 oranges", 19) == 8);
        assert!(fruit_distribution("5 apples and 6 oranges", 21) == 10);
        assert!(fruit_distribution("0 apples and 1 oranges", 3) == 2);
        assert!(fruit_distribution("1 apples and 0 oranges", 3) == 2);
        assert!(fruit_distribution("2 apples and 3 oranges", 100) == 95);
        assert!(fruit_distribution("2 apples and 3 oranges", 5) == 0);
        assert!(fruit_distribution("1 apples and 100 oranges", 120) == 19);
    }


}
