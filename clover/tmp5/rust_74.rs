
fn total_match(lst1: Vec<&str>, lst2: Vec<&str>) -> Vec<&str> {
    let sum1 = lst1.iter().map(|&s| s.len()).sum::<usize>();
    let sum2 = lst2.iter().map(|&s| s.len()).sum::<usize>();

    if sum1 <= sum2 {
        lst1
    } else {
        lst2
    }
}

fn main() {
    let list1 = vec!["apple", "banana", "cherry"];
    let list2 = vec!["pear", "peach", "plum"];

    let result = total_match(list1.iter().map(|&s| s).collect(), list2.iter().map(|&s| s).collect());
    for s in result {
        println!("{}", s);
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
