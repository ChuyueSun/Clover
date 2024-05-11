
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    position: (usize, usize),
    remaining_moves: i32,
    path: Vec<i32>,
    last_value: i32,
}

// Implement ordering for our Node struct to be used in the priority queue
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .path
            .cmp(&self.path)
            .then_with(|| self.remaining_moves.cmp(&other.remaining_moves))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn min_path(grid: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let directions = [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)];
    let n = grid.len();
    let mut heap = BinaryHeap::new();

    for i in 0..n {
        for j in 0..n {
            let start_node = Node {
                position: (i, j),
                remaining_moves: k - 1,
                path: vec![grid[i][j]],
                last_value: grid[i][j],
            };
            heap.push(start_node);
        }
    }

    while let Some(current_node) = heap.pop() {
        if current_node.remaining_moves == 0 {
            return current_node.path;
        }

        let (x, y) = current_node.position;
        for &(dx, dy) in &directions {
            let new_x = (x as i32 + dx) as usize;
            let new_y = (y as i32 + dy) as usize;
            if new_x < n && new_y < n {
                let mut new_path = current_node.path.clone();
                new_path.push(grid[new_x][new_y]);
                let next_node = Node {
                    position: (new_x, new_y),
                    remaining_moves: current_node.remaining_moves - 1,
                    path: new_path,
                    last_value: grid[new_x][new_y],
                };
                heap.push(next_node);
            }
        }
    }

    vec![] // If k is 0, indicating no moves to be made
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_path() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let k = 4;
        assert_eq!(min_path(grid, k), vec![1, 2, 3, 6]);
    }
}

fn main() {
    let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let k = 4;
    let path = min_path(grid, k);
    println!("{:?}", path); // Should print [1, 2, 3, 6]
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
