
fn mean_absolute_deviation(numbers: Vec<f32>) -> f32 {
    let sum: f32 = numbers.iter().sum();
    let count: f32 = numbers.len() as f32;
    let mean: f32 = sum / count;
    let mad: f32 = numbers.iter().map(|&x| (x - mean).abs()).sum::<f32>() / count;
    mad
}

fn main() {
    let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    println!("Mean Absolute Deviation: {}", mean_absolute_deviation(numbers));
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
