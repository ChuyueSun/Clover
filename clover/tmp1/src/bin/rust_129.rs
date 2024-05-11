
use std::cmp::Ordering;

fn min_path(grid: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let n = grid.len();
    let mut min_path = vec![];
    let mut visited = vec![vec![false; n]; n];
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut heap = std::collections::BinaryHeap::new();
    for i in 0..n {
        for j in 0..n {
            heap.push(std::cmp::Reverse((vec![grid[i][j]], (i, j), 1)));
        }
    }

    while let Some(std::cmp::Reverse((path, (x, y), length))) = heap.pop() {
        if length == k as usize {
            min_path = path;
            break;
        }
        if visited[x][y] {
            continue;
        }
        visited[x][y] = true;

        for &(dx, dy) in &directions {
            let new_x = (x as i32 + dx) as usize;
            let new_y = (y as i32 + dy) as usize;
            if new_x < n && new_y < n && !visited[new_x][new_y] {
                let mut new_path = path.clone();
                new_path.push(grid[new_x][new_y]);
                heap.push(std::cmp::Reverse((new_path, (new_x, new_y), length + 1)));
            }
        }
    }

    min_path
}

fn cmp_paths(a: &Vec<i32>, b: &Vec<i32>) -> Ordering {
    for i in 0..a.len() {
        match a[i].cmp(&b[i]) {
            Ordering::Equal => continue,
            non_eq => return non_eq,
        }
    }
    Ordering::Equal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_path() {
        let grid = vec![vec![1, 2], vec![3, 4]];
        let k = 3;
        assert_eq!(min_path(grid, k), vec![1, 2, 3]);

        let grid = vec![vec![9, 7, 5], vec![2, 3, 4], vec![6, 8, 1]];
        let k = 4;
        assert_eq!(min_path(grid, k), vec![1, 3, 2, 3]);
    }
}

fn main() {
    let grid = vec![vec![1, 2], vec![3, 4]];
    let k = 3;
    println!("{:?}", min_path(grid, k));
}
`
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
