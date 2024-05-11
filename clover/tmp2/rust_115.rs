
fn max_fill(grid: Vec<Vec<i32>>, capacity: i32) -> i32 {
    grid.into_iter().map(|well| {
        well.into_iter().sum::<i32>() / capacity + if well.into_iter().sum::<i32>() % capacity > 0 { 1 } else { 0 }
    }).max().unwrap_or(0)
}

fn main() {
    // Example usage:
    let grid = vec![
        vec![1, 1, 0, 0, 1], // Well 1 has 3 units of water
        vec![0, 1, 1, 1, 0], // Well 2 has 3 units of water
        vec![1, 0, 1, 0, 0], // Well 3 has 2 units of water
    ];
    let capacity = 2; // Each bucket has a capacity of 2 units
    let lowerings = max_fill(grid, capacity);
    println!("Number of times to lower the buckets: {}", lowerings);
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
