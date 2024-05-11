
fn get_matrix_triples(n: i32) -> usize {
    let mut a = vec![0; n as usize];
    let mut count = 0;

    // Create array a
    for i in 1..=n {
        a[(i - 1) as usize] = i * i - i + 1;
    }

    // Iterate over all possible triples
    for i in 0..n as usize {
        for j in i+1..n as usize {
            for k in j+1..n as usize {
                if (a[i] + a[j] + a[k]) % 3 == 0 {
                    count += 1;
                }
            }
        }
    }

    count
}

fn main() {
    let n = 10;
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
