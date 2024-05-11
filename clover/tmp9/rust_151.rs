
fn sum_of_squares_of_odds(lst: Vec<f32>) -> f32 {
    lst.iter()
        .filter(|&&x| x.fract() == 0.0 && x > 0.0 && (x as i32) % 2 != 0)
        .map(|&x| x * x)
        .sum()
}

fn main() {
    let numbers = vec![2.0, 3.0, 4.1, -5.0, 7.0];
    println!("Sum of squares of odds is: {}", sum_of_squares_of_odds(numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_the_difference() {
        assert_eq!(double_the_difference(vec![]), 0);
        assert_eq!(double_the_difference(vec![5.0, 4.0]), 25);
        assert_eq!(double_the_difference(vec![0.1, 0.2, 0.3]), 0);
        assert_eq!(double_the_difference(vec![-10.0, -20.0, -30.0]), 0);
        assert_eq!(double_the_difference(vec![-1.0, -2.0, 8.0]), 0);
        assert_eq!(double_the_difference(vec![0.2, 3.0, 5.0]), 34);

        let mut lst = vec![];
        let mut odd_sum = 0;
        for i in -99..100 {
            lst.push(i as f32);
            if i > 0 && i % 2 == 1 {
                odd_sum += i * i;
            }
        }
        assert_eq!(double_the_difference(lst), odd_sum);
    }

}
