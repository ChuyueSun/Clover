
fn parse_music(music_string: String) -> Vec<i32> {
    let mut beats = Vec::new();
    let mut chars = music_string.chars().peekable();
    
    while let Some(c) = chars.next() {
        match c {
            'o' => {
                if chars.peek() == Some(&'|') {
                    chars.next();
                    beats.push(2);
                } else {
                    beats.push(4);
                }
            },
            '.' => {
                if chars.peek() == Some(&'|') {
                    chars.next();
                    beats.push(1);
                }
            },
            _ => {}
        }
    }
    beats
}

fn main() {
    // Example usage:
    let music = "o o| .| o| o".to_string();
    let beats = parse_music(music);
    println!("{:?}", beats); // Should print: [4, 2, 1, 2, 4]
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
