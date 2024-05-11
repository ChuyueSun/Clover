
fn car_race_collision(n: i32) -> i32 {
    // Since each car in the group that's moving left to right will inevitably collide
    // with each car in the group that's moving right to left, the total number of
    // collisions is simply the product of the number of cars in each group.
    n * n
}

fn main() {
    // Example usage:
    let collisions = car_race_collision(5);
    println!("Total collisions: {}", collisions);
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
