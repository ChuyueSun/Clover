
fn is_bored(s: &str) -> i32 {
    s.split(&['.', '?', '!'][..])
        .map(str::trim)
        .filter(|sentence| sentence.starts_with("I ") || sentence == "I")
        .count() as i32
}

fn main() {
    // Example usage:
    let test_str = "I am getting bored. But I don't know why! Is it normal? I think so.";
    println!("The count of boredom is: {}", is_bored(test_str));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_bored() {
        assert!(is_bored("Hello world") == 0);
        assert!(is_bored("Is the sky blue?") == 0);
        assert!(is_bored("I love It !") == 1);
        assert!(is_bored("bIt") == 0);
        assert!(is_bored("I feel good today. I will be productive. will kill It") == 2);
        assert!(is_bored("You and I are going for a walk") == 0);
    }

}
