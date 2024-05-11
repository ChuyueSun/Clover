
fn compare(game: Vec<i32>, guess: Vec<i32>) -> Vec<i32> {
    game.iter().zip(guess.iter()).map(|(&g, &gs)| (g - gs).abs()).collect()
}

fn main() {
    // Example usage:
    let game_scores = vec![10, 24, 18, 30];
    let guesses = vec![12, 18, 18, 29];
    let result_differences = compare(game_scores, guesses);
    println!("{:?}", result_differences); // Should print out the differences in guesses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare() {
        assert_eq!(
            compare(vec![1, 2, 3, 4, 5, 1], vec![1, 2, 3, 4, 2, -2]),
            vec![0, 0, 0, 0, 3, 3]
        );
        assert_eq!(
            compare(vec![0, 5, 0, 0, 0, 4], vec![4, 1, 1, 0, 0, -2]),
            vec![4, 4, 1, 0, 0, 6]
        );
        assert_eq!(
            compare(vec![1, 2, 3, 4, 5, 1], vec![1, 2, 3, 4, 2, -2]),
            vec![0, 0, 0, 0, 3, 3]
        );
        assert_eq!(
            compare(vec![0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0]),
            vec![0, 0, 0, 0, 0, 0]
        );
        assert_eq!(compare(vec![1, 2, 3], vec![-1, -2, -3]), vec![2, 4, 6]);
        assert_eq!(
            compare(vec![1, 2, 3, 5], vec![-1, 2, 3, 4]),
            vec![2, 0, 0, 1]
        );
    }

}
