
fn max_fill(grid: Vec<Vec<i32>>, capacity: i32) -> i32 {
    grid.into_iter().map(|well| {
        // Calculate the total units of water in each well
        let total_units: i32 = well.into_iter().sum();
        // Calculate the number of times we need to lower the bucket for each well
        (total_units + capacity - 1) / capacity
    })
    .max()
    .unwrap_or(0)
}

fn main() {
    // Example usage:
    let grid = vec![
        vec![1, 1, 0, 0, 1], // Well with 3 units of water
        vec![0, 1, 1, 1, 0], // Well with 3 units of water
        vec![1, 1, 1, 1, 1], // Well with 5 units of water
    ];
    let capacity = 2;
    let num_times = max_fill(grid, capacity);
    println!("Number of times the buckets need to be lowered: {}", num_times);
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
