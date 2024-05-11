
fn total_match(lst1: Vec<&str>, lst2: Vec<&str>) -> Vec<&str> {
    let lst1_char_count: usize = lst1.iter().map(|s| s.chars().count()).sum();
    let lst2_char_count: usize = lst2.iter().map(|s| s.chars().count()).sum();

    if lst1_char_count <= lst2_char_count {
        lst1
    } else {
        lst2
    }
}

fn main() {
    // Example usage:
    let lst_a = vec!["apple", "banana", "cherry"];
    let lst_b = vec!["dragonfruit", "elderberry", "fig", "grape"];

    let shorter_list = total_match(lst_a, lst_b);
    for word in shorter_list {
        println!("{word}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_match() {
        let v_empty: Vec<String> = vec![];
        assert!(total_match(vec![], vec![]) == v_empty);
        assert!(total_match(vec!["hi", "admin"], vec!["hi", "hi"]) == vec!["hi", "hi"]);
        assert!(
            total_match(vec!["hi", "admin"], vec!["hi", "hi", "admin", "project"])
                == vec!["hi", "admin"]
        );
        assert!(total_match(vec!["4"], vec!["1", "2", "3", "4", "5"]) == vec!["4"]);
        assert!(total_match(vec!["hi", "admin"], vec!["hI", "Hi"]) == vec!["hI", "Hi"]);
        assert!(total_match(vec!["hi", "admin"], vec!["hI", "hi", "hi"]) == vec!["hI", "hi", "hi"]);
        assert!(total_match(vec!["hi", "admin"], vec!["hI", "hi", "hii"]) == vec!["hi", "admin"]);
        assert!(total_match(vec![], vec!["this"]) == v_empty);
        assert!(total_match(vec!["this"], vec![]) == v_empty);
    }

}
