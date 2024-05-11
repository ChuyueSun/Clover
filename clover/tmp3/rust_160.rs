
use std::collections::VecDeque;

fn do_algebra(operator: Vec<&str>, operand: Vec<i32>) -> i32 {
    let mut ops = VecDeque::from(operator);
    let mut nums = VecDeque::from(operand);
    let mut result = nums.pop_front().unwrap();
    
    while let (Some(op), Some(num)) = (ops.pop_front(), nums.pop_front()) {
        result = match op {
            "+" => result + num,
            "-" => result - num,
            "*" => result * num,
            "//" => result / num,
            "**" => result.pow(num as u32),
            _ => panic!("Unsupported operation"),
        };
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_algebra() {
        assert_eq!(do_algebra(vec!["+", "*", "//", "**"], vec![2, 3, 4, 2, 3]), 1296);
        assert_eq!(do_algebra(vec!["-", "*"], vec![5, 2, 3]), 9);
        assert_eq!(do_algebra(vec!["**", "+"], vec![2, 3, 4]), 64);
    }
}

fn main() {
    let result = do_algebra(vec!["+", "*", "//", "**"], vec![2, 3, 4, 2, 3]);
    println!("The calculated result is: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_algebra() {
        assert_eq!(do_algebra(vec!["**", "*", "+"], vec![2, 3, 4, 5]), 37);
        assert_eq!(do_algebra(vec!["+", "*", "-"], vec![2, 3, 4, 5]), 9);
        assert_eq!(do_algebra(vec!["//", "*"], vec![7, 3, 4]), 8);
    }


}
