
fn is_bored(s: &str) -> u32 {
    s.split(|c| c == '.' || c == '?' || c == '!')
        .filter(|sentence| sentence.trim_start().starts_with("I "))
        .count() as u32
}

fn main() {
    let text = "I am feeling bored today. Are you bored too? I think I will go for a walk! I enjoy walking.";
    println!("The number of boredoms is: {}", is_bored(text));
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
