
fn max_fill(grid: Vec<Vec<i32>>, capacity: i32) -> i32 {
    let mut total_fill_count = 0;
    for well in grid.iter() {
        let well_water: i32 = well.iter().sum();
        total_fill_count += (well_water as f32 / capacity as f32).ceil() as i32;
    }
    total_fill_count
}

fn main() {
    // Example usage:
    let wells = vec![
        vec![1, 1, 1, 0, 0], // 3 units of water in this well
        vec![1, 1, 0, 0, 0],  // 2 units of water in this well
        vec![1, 1, 1, 1, 0],  // 4 units of water in this well
    ];
    let bucket_capacity = 2; // Each bucket has the capacity of 2 units
    let number_of_times = max_fill(wells, bucket_capacity);
    
    println!("Number of times buckets need to be lowered: {}", number_of_times);
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
