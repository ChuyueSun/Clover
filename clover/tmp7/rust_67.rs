
fn fruit_distribution(s: &str, n: i32) -> i32 {
    let total_of_apples_and_oranges: i32 = s.parse().unwrap_or(0);
    n - total_of_apples_and_oranges
}

fn main() {
    let fruits_str = "7";
    let total_fruits = 10;
    let mangoes = fruit_distribution(fruits_str, total_fruits);
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
