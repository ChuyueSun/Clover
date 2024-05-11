
fn get_matrix_triples(n: i32) -> i32 {
    // Generate the array using given formula
    let a: Vec<i32> = (1..=n).map(|i| i*i - i + 1).collect();
    
    // Count elements module 3
    let mut count_mod = [0; 3];
    for &val in &a {
        count_mod[(val % 3) as usize] += 1;
    }

    // Calculate the number of triples that sum to a multiple of 3
    let mut triples = 0;
    
    // Count triples of form (0, 0, 0)
    triples += count_mod[0] * (count_mod[0] - 1) * (count_mod[0] - 2) / 6;

    // Count triples of form (1, 1, 1)
    triples += count_mod[1] * (count_mod[1] - 1) * (count_mod[1] - 2) / 6;

    // Count triples of form (2, 2, 2)
    triples += count_mod[2] * (count_mod[2] - 1) * (count_mod[2] - 2) / 6;

    // Count triples of form (0, 1, 2)
    triples += count_mod[0] * count_mod[1] * count_mod[2];

    triples
}

fn main() {
    // Example usage:
    let n = 5;
    println!("Number of triples: {}", get_matrix_triples(n));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_matrix_triples() {
        assert_eq!(get_matrix_triples(5), 1);
        assert_eq!(get_matrix_triples(6), 4);
        assert_eq!(get_matrix_triples(10), 36);
        assert_eq!(get_matrix_triples(100), 53361);
    }

}
