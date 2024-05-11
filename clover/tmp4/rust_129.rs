
fn min_path(grid: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let n = grid.len();
    let mut res: Vec<i32> = vec![];
    
    // Find the starting minimum value in the grid.
    let mut start = (0, 0);
    let mut min_val = i32::MAX;
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] < min_val {
                min_val = grid[i][j];
                start = (i, j);
            }
        }
    }
    
    // Given the start position and the remaining steps, we add to the result.
    let (mut x, mut y) = start;
    res.push(grid[x][y]);
    for _ in 1..k {
        let mut next_val = i32::MAX;
        let mut next_pos = (0, 0);
        
        // Check every possible 4 directions (up, down, left, right)
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dx, dy) in directions {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;
            
            if new_x >= 0 && new_x < n as isize && new_y >= 0 && new_y < n as isize {
                let new_val = grid[new_x as usize][new_y as usize];
                if new_val < next_val {
                    next_val = new_val;
                    next_pos = (new_x as usize, new_y as usize);
                }
            }
        }
        
        // Take next step to the least valued cell
        res.push(next_val);
        x = next_pos.0;
        y = next_pos.1;
    }
    
    res
}

// You can use the following main function to test the min_path function
fn main() {
    let grid = vec![vec![1, 3, 4], vec![5, 2, 6], vec![7, 8, 9]];
    let k = 4;
    let path = min_path(grid, k);
    println!("{:?}", path); // This will print the path.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_path() {
        assert_eq!(
            min_path(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 3),
            vec![1, 2, 1]
        );
        assert_eq!(
            min_path(vec![vec![5, 9, 3], vec![4, 1, 6], vec![7, 8, 2]], 1),
            vec![1]
        );
        assert_eq!(
            min_path(
                vec![
                    vec![1, 2, 3, 4],
                    vec![5, 6, 7, 8],
                    vec![9, 10, 11, 12],
                    vec![13, 14, 15, 16]
                ],
                4
            ),
            vec![1, 2, 1, 2]
        );
        assert_eq!(
            min_path(
                vec![
                    vec![6, 4, 13, 10],
                    vec![5, 7, 12, 1],
                    vec![3, 16, 11, 15],
                    vec![8, 14, 9, 2]
                ],
                7
            ),
            vec![1, 10, 1, 10, 1, 10, 1]
        );
        assert_eq!(
            min_path(
                vec![
                    vec![8, 14, 9, 2],
                    vec![6, 4, 13, 15],
                    vec![5, 7, 1, 12],
                    vec![3, 10, 11, 16]
                ],
                5
            ),
            vec![1, 7, 1, 7, 1]
        );
        assert_eq!(
            min_path(
                vec![
                    vec![11, 8, 7, 2],
                    vec![5, 16, 14, 4],
                    vec![9, 3, 15, 6],
                    vec![12, 13, 10, 1]
                ],
                9
            ),
            vec![1, 6, 1, 6, 1, 6, 1, 6, 1]
        );
        assert_eq!(
            min_path(
                vec![
                    vec![12, 13, 10, 1],
                    vec![9, 3, 15, 6],
                    vec![5, 16, 14, 4],
                    vec![11, 8, 7, 2]
                ],
                12
            ),
            vec![1, 6, 1, 6, 1, 6, 1, 6, 1, 6, 1, 6]
        );
        assert_eq!(
            min_path(vec![vec![2, 7, 4], vec![3, 1, 5], vec![6, 8, 9]], 8),
            vec![1, 3, 1, 3, 1, 3, 1, 3]
        );

        assert_eq!(
            min_path(vec![vec![6, 1, 5], vec![3, 8, 9], vec![2, 7, 4]], 8),
            vec![1, 5, 1, 5, 1, 5, 1, 5]
        );

        assert_eq!(
            min_path(vec![vec![1, 2], vec![3, 4]], 10),
            vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2]
        );

        assert_eq!(
            min_path(vec![vec![1, 3], vec![3, 2]], 10),
            vec![1, 3, 1, 3, 1, 3, 1, 3, 1, 3]
        );
    }

}
