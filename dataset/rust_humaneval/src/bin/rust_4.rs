fn main() {}

/*
 For a given list of input numbers, calculate Mean Absolute Deviation
    around the mean of this dataset.
    Mean Absolute Deviation is the average absolute difference between each
    element and a centerpoint (mean in this case):
    MAD = average | x - x_mean |

*/

fn mean_absolute_deviation(numbers: Vec<f32>) -> f32 {
    let mean: f32 = numbers.iter().fold(0.0, |acc: f32, x: &f32| acc + x) / numbers.len() as f32;
    return numbers.iter().map(|x: &f32| (x - mean).abs()).sum::<f32>() / numbers.len() as f32;
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
