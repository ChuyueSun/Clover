
fn parse_music(music_string: String) -> Vec<i32> {
    let mut notes = Vec::new();
    let mut chars = music_string.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            'o' => {
                // Check if the next character is '|', it means it's a half note
                if chars.peek() == Some(&'|') {
                    chars.next(); // consume the '|'
                    notes.push(2);
                } else {
                    notes.push(4);
                }
            }
            '.' => {
                // Check if the next character is '|', it means it's a quarter note
                if chars.peek() == Some(&'|') {
                    chars.next(); // consume the '|'
                    notes.push(1);
                }
            }
            _ => (),
        }
    }

    notes
}

fn main() {
    let music_string = "o o|. .|o| o .|".to_string();
    let beats = parse_music(music_string);
    
    for beat in beats {
        println!("{}", beat);
    }
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
