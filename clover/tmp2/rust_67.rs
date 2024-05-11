
fn fruit_distribution(s: &str, n: i32) -> i32 {
    let parts: Vec<&str> = s.split_whitespace().collect();
    let (apples, oranges): (i32, i32) = match (parts.get(0), parts.get(2)) {
        (Some(a), Some(o)) => {
            (a.parse::<i32>().unwrap_or(0), o.parse::<i32>().unwrap_or(0))
        }
        _ => (0, 0),
    };
    n - apples - oranges
}

fn main() {
    // Example usage:
    let fruits = fruit_distribution("10 apples and 5 oranges", 20);
    println!("There are {} mangoes in the basket.", fruits);
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
