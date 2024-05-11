
fn fruit_distribution(s: &str, n: i32) -> i32 {
    let parts: Vec<&str> = s.split_whitespace().collect();
    let mut total_apples_oranges = 0;

    for part in parts {
        if let Ok(num) = part.parse::<i32>() {
            total_apples_oranges += num;
        }
    }

    n - total_apples_oranges
}

fn main() {
    let fruits_string = "5 apples and 10 oranges";
    let total_fruits = 20;
    println!("Number of mango fruits: {}", fruit_distribution(fruits_string, total_fruits));
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
