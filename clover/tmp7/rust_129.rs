
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: (usize, usize),
    path: Vec<i32>,
    steps: i32,
}

// We want our priority queue to be a min-heap instead of a max-heap,
// so we implement `Ord` such that the smaller costs get priority.
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.path.cmp(&other.path))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn min_path(grid: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let n = grid.len();
    let mut pq = BinaryHeap::new();

    // We use a min heap to keep track of the exploration status.
    // Insert all possible starting points into the priority queue.
    for i in 0..n {
        for j in 0..n {
            pq.push(State {
                cost: grid[i][j],
                position: (i, j),
                path: vec![grid[i][j]],
                steps: 1,
            });
        }
    }

    // Possible moves: up, down, left, right
    let moves = [(0, 1), (1, 0), (0, -1i32 as usize), (-1i32 as usize, 0)];

    while let Some(State { cost, position: (x, y), mut path, steps }) = pq.pop() {
        // We found the shortest path of length k
        if steps == k {
            return path;
        }
        // Explore the neighboring cells
        for &(dx, dy) in &moves {
            let new_x = x.wrapping_add(dx);
            let new_y = y.wrapping_add(dy);

            if new_x < n && new_y < n {
                let next_cost = grid[new_x][new_y];
                let mut new_path = path.clone();
                new_path.push(next_cost);
                pq.push(State {
                    cost: next_cost,
                    position: (new_x, new_y),
                    path: new_path,
                    steps: steps + 1,
                });
            }
        }
    }

    vec![] // We always have a valid path, so this should never be reached
}

fn main() {
    // Example usage:
    let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let k = 4;
    println!("{:?}", min_path(grid, k));
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
