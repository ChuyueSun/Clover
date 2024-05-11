
fn max_fill(grid: Vec<Vec<i32>>, capacity: i32) -> i32 {
    let mut total_fills = 0;
    
    for well in grid.iter() {
        let water_units: i32 = well.iter().sum();
        
        total_fills += (water_units + capacity - 1) / capacity;
    }

    total_fills
}

fn main() {
    let grid = vec![
        vec![1, 1, 1, 0, 0],
        vec![1, 1, 0],
        vec![1, 1, 1, 1, 1],
    ];
    let capacity = 3;
    
    println!("Number of fills needed: {}", max_fill(grid, capacity));
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
