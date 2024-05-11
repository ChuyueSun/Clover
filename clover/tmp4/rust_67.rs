
fn fruit_distribution(s: &str, n: i32) -> i32 {
    let counts = s.split_whitespace()
        .filter(|p| p.parse::<i32>().is_ok())
        .map(|p| p.parse::<i32>().unwrap())
        .sum::<i32>();
    n - counts
}

fn main() {
    let s = "5 apples and 7 oranges";
    let n = 20;
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
