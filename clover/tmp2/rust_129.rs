
use std::collections::BinaryHeap;

fn min_path(grid: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let n = grid.len() as i32;
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut paths: BinaryHeap<(i32, Vec<i32>)> = BinaryHeap::new();

    // Start from each cell
    for i in 0..n {
        for j in 0..n {
            paths.push((grid[i as usize][j as usize] * -1, vec![grid[i as usize][j as usize]]));
        }
    }
    
    while let Some((_, path)) = paths.pop() {
        if path.len() == k as usize {
            return path;
        }
        let last_val = *path.last().unwrap();
        let (i, j) = grid.iter().enumerate()
            .flat_map(|(row_idx, row)| row.iter().enumerate().map(move |(col_idx, &val)| (row_idx, col_idx, val)))
            .find(|&(_, _, val)| val == last_val)
            .map(|(row_idx, col_idx, _)| (row_idx as i32, col_idx as i32))
            .unwrap();
        
        for &(di, dj) in &directions {
            let new_i = i + di;
            let new_j = j + dj;
            if new_i >= 0 && new_i < n && new_j >= 0 && new_j < n {
                let mut new_path = path.clone();
                new_path.push(grid[new_i as usize][new_j as usize]);
                paths.push((*new_path.last().unwrap() * -1, new_path));
            }
        }
    }
    
    vec![] // fallback to satisfy return type if something unexpected happens
}

// Example usage:
// let grid = vec![vec![1, 2], vec![3, 4]];
// let k = 3;
// let path = min_path(grid, k);

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
