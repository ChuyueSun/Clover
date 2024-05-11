
fn max_fill(grid: Vec<Vec<i32>>, capacity: i32) -> i32 {
    let mut max_fill_count = 0;
    
    for well in grid.iter() {
        let total_water: i32 = well.iter().sum();
        let fill_count = (total_water as f32 / capacity as f32).ceil() as i32;
        max_fill_count = std::cmp::max(max_fill_count, fill_count);
    }

    max_fill_count
}

fn main() {
    let grid = vec![
        vec![1, 1, 0, 0, 1], // 3 units of water
        vec![0, 1, 1, 1, 0], // 3 units of water
        vec![1, 0, 0, 1, 1], // 3 units of water
    ];
    let capacity = 2;

    println!("Number of times to lower the buckets: {}", max_fill(grid, capacity));
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
