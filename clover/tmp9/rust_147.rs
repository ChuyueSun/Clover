
fn get_matrix_triples(n: i32) -> i32 {
    fn a(i: i32) -> i32 {
        i * i - i + 1
    }

    let mut count_0 = 0;
    let mut count_1 = 0;
    let mut count_2 = 0;

    // Count how many times each remainder appears when an element is divided by 3
    for i in 1..=n {
        match a(i) % 3 {
            0 => count_0 += 1,
            1 => count_1 += 1,
            2 => count_2 += 1,
            _ => (),
        }
    }

    // Calculate the number of valid triples where their sum is a multiple of 3
    // Taking 3 elements all of which when divided by 3 leave remainder 0
    let comb_0 = count_0 * (count_0 - 1) * (count_0 - 2) / 6;
    // Taking 3 elements all of which when divided by 3 leave different remainders
    let comb_1 = count_0 * count_1 * count_2;
    // Taking 3 elements all of which when divided by 3 leave remainder 1
    let comb_2 = count_1 * (count_1 - 1) * (count_1 - 2) / 6;
    // Taking 3 elements all of which when divided by 3 leave remainder 2
    let comb_3 = count_2 * (count_2 - 1) * (count_2 - 2) / 6;

    // Sum up all the possible combinations
    comb_0 + comb_1 + comb_2 + comb_3
}

fn main() {
    let n = 10;

    // Test the function with 'n'
    let result = get_matrix_triples(n);
    println!("{}", result); // The output should be the number of valid triples
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
