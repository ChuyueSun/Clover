
fn bf(planet1: &str, planet2: &str) -> Vec<&'static str> {
    let planets = vec![
        "Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune",
    ];

    let (i1, i2) = (
        planets.iter().position(|&p| p == planet1),
        planets.iter().position(|&p| p == planet2),
    );

    match (i1, i2) {
        (Some(index1), Some(index2)) if index1 < index2 => {
            planets[index1 + 1..index2].to_vec()
        }
        (Some(index1), Some(index2)) if index1 > index2 => {
            planets[index2 + 1..index1].to_vec()
        }
        _ => vec![],
    }
}

fn main() {
    // Examples
    println!("{:?}", bf("Mercury", "Jupiter")); // Returns planets between Mercury and Jupiter
    println!("{:?}", bf("Neptune", "Venus")); // Returns planets between Venus and Neptune
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
