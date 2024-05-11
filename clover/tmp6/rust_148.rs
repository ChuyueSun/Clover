
fn bf(planet1: &str, planet2: &str) -> Vec<&str> {
    let planets = vec![
        "Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune",
    ];
    let mut between_planets = Vec::new();

    let (start_idx, end_idx) = if let (Some(start), Some(end)) = (planets.iter().position(|&p| p == planet1), planets.iter().position(|&p| p == planet2)) {
        if start < end {
            (start, end)
        } else {
            (end, start)
        }
    } else {
        // Return empty vector if planet names are incorrect.
        return between_planets;
    };

    // Starting from the next planet after `start_idx` up to (but not including) `end_idx`.
    for i in start_idx + 1..end_idx {
        between_planets.push(planets[i]);
    }

    between_planets
}

fn main() {
    // Example usage:
    let planet1 = "Earth";
    let planet2 = "Saturn";
    let result = bf(planet1, planet2);
    println!("Planets between {} and {}: {:?}", planet1, planet2, result);
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
