
fn parse_music(music_string: String) -> Vec<i32> {
    let mut beats = Vec::new();
    let mut partial_note = String::new();

    for c in music_string.chars() {
        partial_note.push(c);
        
        match partial_note.as_str() {
            "o" => {
                beats.push(4);
                partial_note.clear();
            },
            "o|" => {
                beats.push(2);
                partial_note.clear();
            },
            ".|" => {
                beats.push(1);
                partial_note.clear();
            },
            _ => {}
        }
    }

    beats
}

fn main() {
    let music = String::from("o o|.||.");
    let beats = parse_music(music);
    println!("{:?}", beats);
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
