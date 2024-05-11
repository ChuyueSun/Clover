
fn numerical_letter_grade(grades: Vec<f64>) -> Vec<String> {
    grades.iter().map(|&gpa| {
        if gpa == 4.0 {
            "A+".into()
        } else if gpa > 3.7 {
            "A".into()
        } else if gpa > 3.3 {
            "A-".into()
        } else if gpa > 3.0 {
            "B+".into()
        } else if gpa > 2.7 {
            "B".into()
        } else if gpa > 2.3 {
            "B-".into()
        } else if gpa > 2.0 {
            "C+".into()
        } else if gpa > 1.7 {
            "C".into()
        } else if gpa > 1.3 {
            "C-".into()
        } else if gpa > 1.0 {
            "D+".into()
        } else if gpa > 0.7 {
            "D".into()
        } else if gpa > 0.0 {
            "D-".into()
        } else {
            "E".into()
        }
    }).collect()
}

fn main() {
    // Example usage:
    let grades = vec![4.0, 3.85, 3.5, 3.2, 0.0, 2.5];
    let letter_grades = numerical_letter_grade(grades);

    for grade in letter_grades {
        println!("{}", grade);
    }
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
