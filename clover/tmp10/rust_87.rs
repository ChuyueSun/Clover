
fn get_x_coordinates(lst: Vec<Vec<i32>>, x: i32) -> Vec<(usize, usize)> {
    let mut coordinates = Vec::new();

    for (row_index, row) in lst.iter().enumerate() {
        for (col_index, &item) in row.iter().enumerate() {
            if item == x {
                coordinates.push((row_index, row.len() - 1 - col_index));
            }
        }
    }

    coordinates.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| b.1.cmp(&a.1)));
    coordinates.iter().map(|&(r, c)| (r, row.len() - 1 - c)).collect()
}

fn main() {
    // Example usage
    let lst = vec![
        vec![1, 2, 3, 4],
        vec![5, 2, 3, 1],
        vec![1, 1, 1],
        vec![3, 7, 1, 9, 1],
    ];
    let x = 1;
    let coordinates = get_x_coordinates(lst, x);
    println!("{:?}", coordinates);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_row() {
        assert!(
            get_row(
                vec![
                    vec![1, 2, 3, 4, 5, 6],
                    vec![1, 2, 3, 4, 1, 6],
                    vec![1, 2, 3, 4, 5, 1]
                ],
                1
            ) == vec![vec![0, 0], vec![1, 0], vec![1, 4], vec![2, 0], vec![2, 5]]
        );
        assert!(
            get_row(
                vec![
                    vec![1, 2, 3, 4, 5, 6],
                    vec![1, 2, 3, 4, 5, 6],
                    vec![1, 2, 3, 4, 5, 6],
                    vec![1, 2, 3, 4, 5, 6],
                    vec![1, 2, 3, 4, 5, 6],
                    vec![1, 2, 3, 4, 5, 6]
                ],
                2
            ) == vec![
                vec![0, 1],
                vec![1, 1],
                vec![2, 1],
                vec![3, 1],
                vec![4, 1],
                vec![5, 1]
            ]
        );
        assert!(
            get_row(
                vec![
                    vec![1, 2, 3, 4, 5, 6],
                    vec![1, 2, 3, 4, 5, 6],
                    vec![1, 1, 3, 4, 5, 6],
                    vec![1, 2, 1, 4, 5, 6],
                    vec![1, 2, 3, 1, 5, 6],
                    vec![1, 2, 3, 4, 1, 6],
                    vec![1, 2, 3, 4, 5, 1]
                ],
                1
            ) == vec![
                vec![0, 0],
                vec![1, 0],
                vec![2, 0],
                vec![2, 1],
                vec![3, 0],
                vec![3, 2],
                vec![4, 0],
                vec![4, 3],
                vec![5, 0],
                vec![5, 4],
                vec![6, 0],
                vec![6, 5]
            ]
        );
        let v: Vec<Vec<i32>> = vec![];
        assert!(get_row(vec![], 1) == v);
        assert!(get_row(vec![vec![1]], 2) == v);
        assert!(get_row(vec![vec![], vec![1], vec![1, 2, 3]], 3) == vec![vec![2, 2]]);
    }

}