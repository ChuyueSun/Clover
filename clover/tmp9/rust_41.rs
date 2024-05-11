
fn car_race_collision(n: i32) -> i32 {
    // Since all cars are moving at the same speed and the line is infinitely long,
    // every car moving from left to right will eventually collide with each car coming
    // from right to left. So, the total number of collisions will simply be the number 
    // of cars moving in one direction multiplied by the number of cars moving in the
    // opposite direction, which is n * n.
    n * n
}

fn main() {
    let n = 5; // An example number of cars going in each direction.
    let collisions = car_race_collision(n);
    println!("The number of collisions is: {}", collisions);
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
