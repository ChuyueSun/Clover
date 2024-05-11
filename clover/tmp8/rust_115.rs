
fn max_fill(grid: Vec<Vec<i32>>, capacity: i32) -> i32 {
    let mut fill_count = 0;

    for well in grid {
        let units_of_water: i32 = well.iter().sum();
        fill_count += (units_of_water as f32 / capacity as f32).ceil() as i32;
    }

    fill_count
}

fn main() {
    // Example usage:
    // let grid: Vec<Vec<i32>> = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    // let capacity: i32 = 3;
    // println!("Number of times to lower the buckets: {}", max_fill(grid, capacity));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_fill() {
        assert!(
            max_fill(
                vec![vec![0, 0, 1, 0], vec![0, 1, 0, 0], vec![1, 1, 1, 1]],
                1
            ) == 6
        );
        assert!(
            max_fill(
                vec![
                    vec![0, 0, 1, 1],
                    vec![0, 0, 0, 0],
                    vec![1, 1, 1, 1],
                    vec![0, 1, 1, 1]
                ],
                2
            ) == 5
        );
        assert!(max_fill(vec![vec![0, 0, 0], vec![0, 0, 0]], 5) == 0);
        assert!(max_fill(vec![vec![1, 1, 1, 1], vec![1, 1, 1, 1]], 2) == 4);
        assert!(max_fill(vec![vec![1, 1, 1, 1], vec![1, 1, 1, 1]], 9) == 2);
    }

}
