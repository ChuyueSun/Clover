
fn min_path(grid: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let n = grid.len();
    let (mut x, mut y) = (0, 0);

    // Find the minimum value's coordinates
    for (i, row) in grid.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == 1 {
                x = i;
                y = j;
                break;
            }
        }
    }
    
    let mut path = Vec::new();
    let mut steps = 0;
    let mut dx: i32;
    let mut dy: i32;
    
    // Movement offsets (up, right, down, left)
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    
    while steps < k {
        path.push(grid[x][y]);
        steps += 1;

        // Select next step
        let mut next_value = std::i32::MAX;
        let mut next_dir = (0, 0);

        for &(dx, dy) in directions.iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            
            if nx >= 0 && nx < n as i32 && ny >= 0 && ny < n as i32 {
                if grid[nx as usize][ny as usize] < next_value {
                    next_value = grid[nx as usize][ny as usize];
                    next_dir = (dx, dy);
                }
            }
        }

        // Make a step if possible
        dx = next_dir.0;
        dy = next_dir.1;
        
        if steps < k {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < n as i32 && ny >= 0 && ny < n as i32 {
                x = nx as usize;
                y = ny as usize;
            }
        }
    }

    path
}

// Below is test code, which you can use to check the correctness of the function,
// but it should not be included as part of the solution to run in production.

// fn main(){
//     let grid = vec![
//         vec![1, 2, 3],
//         vec![8, 9, 4],
//         vec![7, 6, 5],
//     ];
//     let k = 4;
//     let path = min_path(grid, k);
//     println!("{:?}", path); // Should print [1, 2, 3, 4]
// }

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
