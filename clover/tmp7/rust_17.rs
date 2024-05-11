
fn parse_music(music_string: String) -> Vec<i32> {
    let mut beats = Vec::new();
    let mut it = music_string.chars().peekable();

    while let Some(c) = it.next() {
        match c {
            'o' => {
                if it.peek() == Some(&'|') {
                    beats.push(2);
                    it.next();
                } else {
                    beats.push(4);
                }
            }
            '.' => {
                if it.peek() == Some(&'|') {
                    beats.push(1);
                    it.next();
                }
            }
            _ => {}
        }
    }

    beats
}

fn main() {
    let music_string = String::from("o o|. o| .| o");
    let beats = parse_music(music_string);
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
