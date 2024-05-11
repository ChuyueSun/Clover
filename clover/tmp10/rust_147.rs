
fn get_matrix_triples(n: i32) -> i32 {
    let mut triples_count = 0;

    // Create the integer array a of length n
    let mut a = Vec::new();
    for i in 1..=n {
        a.push(i*i - i + 1);
    }

    // Find the number of triples where the sum of the elements is a multiple of 3
    for i in 0..n as usize {
        for j in i+1..n as usize {
            for k in j+1..n as usize {
                if (a[i] + a[j] + a[k]) % 3 == 0 {
                    triples_count += 1;
                }
            }
        }
    }

    triples_count
}

fn main() {
    // You can call the function with a certain n to test it
    let n = 4; // For example
    let result = get_matrix_triples(n);
    println!("Number of triples: {}", result);
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
