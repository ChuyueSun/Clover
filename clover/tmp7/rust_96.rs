
fn count_up_to(n: i32) -> Vec<i32> {
    let mut primes = vec![];
    let mut is_prime;

    for num in 2..n {
        is_prime = true;
        for divisor in 2..((num as f64).sqrt() as i32 + 1) {
            if num % divisor == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(num);
        }
    }

    primes
}

fn main() {
    let primes = count_up_to(10);
    println!("{:?}", primes); // Should print [2, 3, 5, 7]
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_count_up_to() {
        assert!(count_up_to(5) == vec![2, 3]);
        assert!(count_up_to(6) == vec![2, 3, 5]);
        assert!(count_up_to(7) == vec![2, 3, 5]);
        assert!(count_up_to(10) == vec![2, 3, 5, 7]);
        assert!(count_up_to(0) == vec![]);
        assert!(count_up_to(22) == vec![2, 3, 5, 7, 11, 13, 17, 19]);
        assert!(count_up_to(1) == vec![]);
        assert!(count_up_to(18) == vec![2, 3, 5, 7, 11, 13, 17]);
        assert!(count_up_to(47) == vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43]);
        assert!(
            count_up_to(101)
                == vec![
                    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73,
                    79, 83, 89, 97
                ]
        );
    }

}
