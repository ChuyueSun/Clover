
use std::collections::BinaryHeap;

fn min_path(grid: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let n = grid.len() as i32;
    let mut heap: BinaryHeap<(i32, i32, i32, Vec<i32>)> = BinaryHeap::new();
    let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    // Push initial positions into the heap with their path included.
    for i in 0..n {
        for j in 0..n {
            heap.push((-grid[i as usize][j as usize], i, j, vec![grid[i as usize][j as usize]]));
        }
    }

    while let Some((_, x, y, path)) = heap.pop() {
        // We have found the shortest path of length k
        if path.len() == k as usize {
            return path;
        }

        // Explore the neighbors
        for &(dx, dy) in &directions {
            let new_x = x + dx;
            let new_y = y + dy;

            if new_x >= 0 && new_x < n && new_y >= 0 && new_y < n {
                let mut new_path = path.clone();
                new_path.push(grid[new_x as usize][new_y as usize]);
                heap.push((
                    -grid[new_x as usize][new_y as usize],
                    new_x,
                    new_y,
                    new_path,
                ));
            }
        }
    }

    vec![] // Although the problem statement guarantees a solution, this satisfies the function signature.
}

fn main() {
    // Example usage (uncomment and fill grid and k to test):
    // let grid = vec![vec![1, 2], vec![3, 4]]; // Replace with actual grid
    // let k = 3; // Replace with actual k
    // let result = min_path(grid, k);
    // println!("{:?}", result); // Should print a vector
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
