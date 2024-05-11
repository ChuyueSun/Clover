
fn fruit_distribution(s: &str, n: i32) -> i32 {
    if let Ok(total_apples_oranges) = s.parse::<i32>() {
        n - total_apples_oranges
    } else {
        -1 // Return -1 or any other error code if the string can't be parsed to a number
    }
}

fn main() {
    // Example usage
    let fruit_string = "30";
    let total_fruits = 50;
    let mangoes = fruit_distribution(fruit_string, total_fruits);
    println!("Number of mangoes: {}", mangoes);
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
