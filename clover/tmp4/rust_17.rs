
fn parse_music(music_string: String) -> Vec<i32> {
    let mut beats = Vec::new();
    let mut chars = music_string.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            'o' => match chars.peek() {
                Some('|') => {
                    chars.next(); // consume the '|' character
                    beats.push(2);
                },
                _ => beats.push(4),
            },
            '.' => match chars.peek() {
                Some('|') => {
                    chars.next(); // consume the '|' character
                    beats.push(1);
                },
                _ => (),
            },
            _ => (),
        }
    }

    beats
}

fn main() {
    let music = "o o| .| o| .| o".to_string();
    let beats = parse_music(music);

    println!("{:?}", beats); // This should print: [4, 2, 1, 2, 1, 4]
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
