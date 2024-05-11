
fn numerical_letter_grade(grades: Vec<f64>) -> Vec<String> {
    grades.iter().map(|&gpa| {
        match gpa {
            gpa if gpa >= 4.0 => String::from("A+"),
            gpa if gpa > 3.7 => String::from("A"),
            gpa if gpa > 3.3 => String::from("A-"),
            gpa if gpa > 3.0 => String::from("B+"),
            gpa if gpa > 2.7 => String::from("B"),
            gpa if gpa > 2.3 => String::from("B-"),
            gpa if gpa > 2.0 => String::from("C+"),
            gpa if gpa > 1.7 => String::from("C"),
            gpa if gpa > 1.3 => String::from("C-"),
            gpa if gpa > 1.0 => String::from("D+"),
            gpa if gpa > 0.7 => String::from("D"),
            gpa if gpa > 0.0 => String::from("D-"),
            0.0 => String::from("E"),
            _ => String::from("Invalid GPA"),
        }
    }).collect()
}

fn main() {
    let gpas = vec![3.9, 2.5, 1.2, 4.0, 0.0];
    let letter_grades = numerical_letter_grade(gpas);
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
