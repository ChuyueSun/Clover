
fn car_race_collision(n: i32) -> i32 {
    // Since each car moving left to right will eventually collide with each car moving right to left,
    // and there are n cars in each direction, there will be n * n total collisions.
    n * n
}

fn main() {
    // Example usage:
    let collisions = car_race_collision(5);
    println!("Number of collisions: {}", collisions);
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
