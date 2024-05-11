
fn mean_absolute_deviation(numbers: Vec<f32>) -> f32 {
    if numbers.is_empty() {
        return 0.0;
    }
    let mean: f32 = numbers.iter().sum::<f32>() / numbers.len() as f32;
    let mad: f32 = numbers.iter().map(|&x| (x - mean).abs()).sum::<f32>() / numbers.len() as f32;
    mad
}

fn main() {
    let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let mad = mean_absolute_deviation(numbers);
    println!("Mean Absolute Deviation: {}", mad);
}

#[cfg(test)]
mod tests {
    use super::*;

  #[test]
    fn test_mean_absolute_deviation() {
        assert!(mean_absolute_deviation(vec![1.0, 2.0, 3.0]) - 2.0 / 3.0 < 0.000001);
        assert!(mean_absolute_deviation(vec![1.0, 2.0, 3.0, 4.0]) - 1.0 < 0.000001);
        assert!(mean_absolute_deviation(vec![1.0, 2.0, 3.0, 4.0, 5.0]) - 6.0 / 5.0 < 0.000001);
    }

}
