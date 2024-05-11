
fn bf(planet1: &str, planet2: &str) -> Vec<&str> {
    let planets = vec![
        "Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune",
    ];
    
    let idx1 = planets.iter().position(|&p| p == planet1);
    let idx2 = planets.iter().position(|&p| p == planet2);

    match (idx1, idx2) {
        (Some(index1), Some(index2)) if index1 != index2 => {
            let (start, end) = if index1 < index2 {
                (index1, index2)
            } else {
                (index2, index1)
            };
            planets[start + 1..end].to_vec()
        }
        _ => vec![],
    }
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
