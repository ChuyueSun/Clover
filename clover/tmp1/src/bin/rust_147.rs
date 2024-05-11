
fn get_matrix_triples(n: i32) -> i32 {
    let mut a = Vec::new();
    for i in 1..=n {
        a.push(i * i - i + 1);
    }

    let mut count = 0;
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
    // Example usage:
    // let n = 5;
    // let number_of_triples = get_matrix_triples(n);
    // println!("Number of triples: {}", number_of_triples);
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
