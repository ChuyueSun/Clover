
fn fruit_distribution(s: &str, n: i32) -> i32 {
    let parts: Vec<&str> = s.split_whitespace().collect();
    let apples_and_oranges = parts.iter()
        .filter_map(|&num| num.parse::<i32>().ok())
        .sum::<i32>();
    n - apples_and_oranges
}

fn main() {
    let s = "5 apples and 2 oranges";
    let n = 10;
    println!("Number of mangoes: {}", fruit_distribution(s, n));
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
