
fn car_race_collision(n: i32) -> i32 {
    n * n
}

fn main() {
    let number_of_collisions = car_race_collision(5);
    println!("Number of collisions: {}", number_of_collisions);
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_car_race_collision() {
        assert!(car_race_collision(2) == 4);
        assert!(car_race_collision(3) == 9);
        assert!(car_race_collision(4) == 16);
        assert!(car_race_collision(8) == 64);
        assert!(car_race_collision(10) == 100);
    }

}
