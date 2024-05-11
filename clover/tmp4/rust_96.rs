
fn count_up_to(n: i32) -> Vec<i32> {
    if n <= 2 {
        return Vec::new();
    }
    
    let mut primes = vec![2];
    for num in 3..n {
        if is_prime(num) {
            primes.push(num);
        }
    }
    primes
}

fn is_prime(x: i32) -> bool {
    if x <= 1 {
        return false;
    }
    if x == 2 {
        return true;
    }
    if x % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= x {
        if x % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

fn main() {
    let n = 10;
    println!("Primes up to {}: {:?}", n, count_up_to(n));
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
