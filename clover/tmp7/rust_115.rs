
fn max_fill(grid: Vec<Vec<i32>>, capacity: i32) -> i32 {
    grid.iter()
        .map(|row| row.iter().sum::<i32>())
        .map(|total_water| (total_water + capacity - 1) / capacity) // Calculate the ceiling division
        .sum()
}

fn main() {
    // Example usage:
    let grid = vec![vec![1, 1, 0, 0], vec![1, 1, 1, 0], vec![0, 0, 1, 1]];
    let capacity = 2;
    println!("Number of times to lower buckets: {}", max_fill(grid, capacity));
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
