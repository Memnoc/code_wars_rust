fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    todo!()
}

// TODO:
// senior at least 55 years old with handicap greater than 77
// >= 55 && > 77
// Handicap range is -2 to 26 (lower is better)
// input is an integer for the person age and integre for handicap
// 18, 20 = "Open"
// 55, 21 = "Senior"
fn main() {
    println!("=== Categorise New Member ===");
}

#[cfg(test)]
mod test {
    use super::open_or_senior;

    #[test]
    fn test_open_or_senior() {
        assert_eq!(
            open_or_senior(vec![(45, 12), (55, 21), (19, -2), (104, 20)]),
            vec!["Open", "Senior", "Open", "Senior"]
        );
        assert_eq!(
            open_or_senior(vec![(3, 12), (55, 1), (91, -2), (54, 23)]),
            vec!["Open", "Open", "Open", "Open"]
        );
    }
}
