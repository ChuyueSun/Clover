
fn fib4(n: i32) -> i32 {
    let mut values = [0, 0, 2, 0];
    if n < 4 {
        return values[n as usize];
    }
    for i in 4..=n {
        let next_value = values[0] + values[1] + values[2] + values[3];
        values[i as usize % 4] = next_value;
    }
    values[n as usize % 4]
}

fn main() {
    // Example usage:
    println!("fib4(5): {}", fib4(5)); // Should print 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib4() {
        assert!(fib4(5) == 4);
        assert!(fib4(8) == 28);
        assert!(fib4(10) == 104);
        assert!(fib4(12) == 386);
    }

}
