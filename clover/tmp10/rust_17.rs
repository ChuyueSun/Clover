
fn parse_music(music_string: String) -> Vec<i32> {
    let mut result = Vec::new();
    let mut characters = music_string.chars().peekable();

    while let Some(c) = characters.next() {
        match c {
            'o' => {
                if characters.peek() == Some(&'|') {
                    characters.next(); // Consume the '|'
                    result.push(2);
                } else {
                    result.push(4);
                }
            }
            '.' => {
                if characters.peek() == Some(&'|') {
                    characters.next(); // Consume the '|'
                    result.push(1);
                }
            }
            _ => {}
        }
    }

    result
}

fn main() {
    // Example usage
    let music = String::from("o o| o .| o");
    let beats = parse_music(music);
    println!("{:?}", beats); // Should output [4, 2, 4, 1, 4]
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_parse_music() {
        assert!(parse_music(" ".to_string()) == []);
        assert!(parse_music("o o o o".to_string()) == vec![4, 4, 4, 4]);
        assert!(parse_music(".| .| .| .|".to_string()) == vec![1, 1, 1, 1]);
        assert!(parse_music("o| o| .| .| o o o o".to_string()) == vec![2, 2, 1, 1, 4, 4, 4, 4]);
        assert!(parse_music("o| .| o| .| o o| o o|".to_string()) == vec![2, 1, 2, 1, 4, 2, 4, 2]);
    }

}
