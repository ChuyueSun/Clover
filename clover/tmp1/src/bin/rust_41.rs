
fn car_race_collision(n: i32) -> i32 {
    // Since all the cars move at the same speed and are equally spaced,
    // every car moving left to right will eventually collide with every car
    // moving right to left, hence there will be n*n collisions
    n * n
}

fn main() {
    // Example usage:
    let number_of_collisions = car_race_collision(5);
    println!("The number of collisions is: {}", number_of_collisions);
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
