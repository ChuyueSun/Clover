
fn get_matrix_triples(n: i32) -> i32 {
    if n < 3 {
        return 0;
    }

    let mut triples = 0;
    let mut counters = [0; 3]; // To hold counts for values mod 3

    for i in 1..=n {
        let val = i * i - i + 1;
        counters[(val % 3) as usize] += 1;
    }

    // Calculate the number of valid triples
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                // Check if i+j+k is a multiple of 3
                if (i + j + k) % 3 == 0 {
                    // If i, j, k are the same, we have to pick 3 different elements from that binomially
                    if i == j && j == k {
                        triples += counters[i] * (counters[i] - 1) * (counters[i] - 2) / 6;
                    }
                    // If i, j are the same, pick 2 from i/j and 1 from k
                    else if i == j || j == k || i == k {
                        triples += counters[i] * (counters[j] - 1) * counters[k] / 2;
                    }
                    // If all are different, we can just multiply the counts
                    else {
                        triples += counters[i] * counters[j] * counters[k];
                    }
                }
            }
        }
    }

    triples
}

fn main() {
    // Example usage:
    let n = 5;
    let result = get_matrix_triples(n);
    println!("The number of triples for n = {} is {}", n, result);
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
