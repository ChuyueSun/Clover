
/// Counts the number of collisions that will occur between two equal sets of cars
/// traveling in opposite directions on an infinitely long, straight road.
///
/// Arguments:
/// * `n` (i32): The number of cars traveling in each direction.
///
/// Returns:
/// * i32: The total number of collisions.
///
/// Note that because the road is straight and cars are equally spaced, every car
/// traveling in one direction will collide with every car traveling in the opposite
/// direction.
fn car_race_collision(n: i32) -> i32 {
    // Since every car going in one direction will collide with every car going in the opposite direction
    // and there are n cars in each direction, there will be a total of n * n collisions.
    n * n
}

fn main() {
    let number_of_collisions = car_race_collision(5);
    println!("The total number of collisions is: {}", number_of_collisions);
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
