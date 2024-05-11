
const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn min_path(grid: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let n = grid.len() as i32;
    let mut answer = vec![i32::MAX; k as usize];
    let mut path = Vec::new();

    // Function to perform dfs traversal and update the answer path.
    fn dfs(
        row: i32,
        col: i32,
        k: i32,
        n: i32,
        grid: &Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        answer: &mut Vec<i32>,
    ) {
        if k == 0 {
            if path < answer {
                *answer = path.clone();
            }
            return;
        }

        for &(dr, dc) in &DIRECTIONS {
            let new_row = row + dr;
            let new_col = col + dc;
            if new_row >= 0 && new_row < n && new_col >= 0 && new_col < n {
                path.push(grid[new_row as usize][new_col as usize]);
                dfs(new_row, new_col, k - 1, n, grid, path, answer);
                path.pop();
            }
        }
    }

    // Start dfs from each cell in the grid
    for i in 0..n {
        for j in 0..n {
            path.push(grid[i as usize][j as usize]);
            dfs(i, j, k - 1, n, &grid, &mut path, &mut answer);
            path.pop();
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_path() {
        let grid = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        assert_eq!(min_path(grid, 2), vec![1, 2]);

        let grid = vec![
            vec![9, 4, 7],
            vec![1, 5, 3],
            vec![2, 8, 6],
        ];
        assert_eq!(min_path(grid, 3), vec![1, 2, 3]);
    }
}

fn main() {
    // You can test the function here, if you want.
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
