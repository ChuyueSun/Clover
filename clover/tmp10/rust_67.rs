
fn fruit_distribution(s: &str, n: i32) -> i32 {
    let sum_apples_oranges: i32 = s.parse().unwrap_or(0);
    n - sum_apples_oranges
}

fn main() {
    // Example usage:
    let fruits = "20"; // Represents the total number of apples + oranges.
    let total_fruits = 25; // Represents the total number of fruits in the basket.
    let mangoes = fruit_distribution(fruits, total_fruits);

    println!("The number of mangoes in the basket is: {}", mangoes);
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
