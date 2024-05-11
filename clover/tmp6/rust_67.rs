
fn fruit_distribution(s: &str, n: i32) -> i32 {
    let fruits: Vec<&str> = s.split(' ').collect();
    let apples: i32 = fruits[0].parse().unwrap_or(0);
    let oranges: i32 = fruits[1].parse().unwrap_or(0);
    n - (apples + oranges)
}

fn main() {
    // Example usage:
    let basket_str = "10 15"; // 10 apples and 15 oranges
    let total_fruits = 40; // total fruits in the basket
    let mangoes = fruit_distribution(basket_str, total_fruits);
    println!("There are {} mangoes in the basket.", mangoes);
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
