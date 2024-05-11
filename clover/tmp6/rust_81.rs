
fn numerical_letter_grade(grades: Vec<f64>) -> Vec<String> {
    grades
        .into_iter()
        .map(|gpa| {
            if gpa >= 4.0 {
                "A+".to_string()
            } else if gpa > 3.7 {
                "A".to_string()
            } else if gpa > 3.3 {
                "A-".to_string()
            } else if gpa > 3.0 {
                "B+".to_string()
            } else if gpa > 2.7 {
                "B".to_string()
            } else if gpa > 2.3 {
                "B-".to_string()
            } else if gpa > 2.0 {
                "C+".to_string()
            } else if gpa > 1.7 {
                "C".to_string()
            } else if gpa > 1.3 {
                "C-".to_string()
            } else if gpa > 1.0 {
                "D+".to_string()
            } else if gpa > 0.7 {
                "D".to_string()
            } else if gpa > 0.0 {
                "D-".to_string()
            } else {
                "E".to_string()
            }
        })
        .collect()
}

fn main() {
    // Example use case
    let gpa_scores = vec![4.0, 3.5, 2.2, 1.2, 0.0];
    let letter_grades = numerical_letter_grade(gpa_scores);
    println!("{:?}", letter_grades);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numerical_letter_grade() {
        assert!(
            numerical_letter_grade(vec![4.0, 3.0, 1.7, 2.0, 3.5])
                == vec!["A+", "B", "C-", "C", "A-"]
        );
        assert!(numerical_letter_grade(vec![1.2]) == vec!["D+"]);
        assert!(numerical_letter_grade(vec![0.5]) == vec!["D-"]);
        assert!(numerical_letter_grade(vec![0.0]) == vec!["E"]);
        assert!(
            numerical_letter_grade(vec![1.0, 0.3, 1.5, 2.8, 3.3])
                == vec!["D", "D-", "C-", "B", "B+"]
        );
        assert!(numerical_letter_grade(vec![0.0, 0.7]) == vec!["E", "D-"]);
    }

}
