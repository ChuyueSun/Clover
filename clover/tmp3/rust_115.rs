
fn max_fill(grid: Vec<Vec<i32>>, capacity: i32) -> i32 {
    grid.into_iter()
        .map(|well| well.into_iter().sum::<i32>())
        .map(|total_units| (total_units + capacity - 1) / capacity)
        .sum()
}

fn main() {
    // Example usage:
    let grid = vec![
        vec![1, 1, 0, 0, 1], // Well 1 with 3 units of water
        vec![0, 1, 1, 1, 0], // Well 2 with 3 units of water
    ];
    let bucket_capacity = 2;
    let number_of_times = max_fill(grid, bucket_capacity);
    println!("The number of times to lower the buckets is: {}", number_of_times);
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
