
fn bf(planet1: &str, planet2: &str) -> (&'static str, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str) {
    let planets = [
        "Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune",
    ];

    let p1_index = planets.iter().position(|&p| p == planet1);
    let p2_index = planets.iter().position(|&p| p == planet2);

    if let (Some(p1), Some(p2)) = (p1_index, p2_index) {
        if p1 < p2 {
            let result = planets[p1 + 1..p2]
                .iter()
                .cloned()
                .collect::<Vec<&str>>()
                .try_into()
                .unwrap_or_default();
            return result;
        } else if p1 > p2 {
            let result = planets[p2 + 1..p1]
                .iter()
                .cloned()
                .collect::<Vec<&str>>()
                .try_into()
                .unwrap_or_default();
            return result;
        }
    }

    ("", "", "", "", "", "", "")
}

fn main() {
    // Example use
    println!("{:?}", bf("Mercury", "Mars")); // Should return the inner planets between Mercury and Mars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bf() {
        assert_eq!(bf("Jupiter", "Neptune"), vec!["Saturn", "Uranus"]);
        assert_eq!(bf("Earth", "Mercury"), vec!["Venus"]);
        assert_eq!(
            bf("Mercury", "Uranus"),
            vec!["Venus", "Earth", "Mars", "Jupiter", "Saturn"]
        );
        assert_eq!(
            bf("Neptune", "Venus"),
            vec!["Earth", "Mars", "Jupiter", "Saturn", "Uranus"]
        );
        let v_empty: Vec<&str> = vec![];
        assert_eq!(bf("Earth", "Earth"), v_empty);
        assert_eq!(bf("Mars", "Earth"), v_empty);
        assert_eq!(bf("Jupiter", "Makemake"), v_empty);
    }

}
