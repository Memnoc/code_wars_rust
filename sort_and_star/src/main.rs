// HEADER: my solution
fn two_sort(arr: &[&str]) -> String {
    let mut sorted = arr.to_vec();
    sorted.sort();
    sorted.join(" ")
}
// NOTE: sort alphabetically a list of strings
// case-sensitive and based on the ASCII value of the chars
// return the first value
// return must be a string
// return must have *** between each letter
fn main() {
    println!("=== Sort and Star ===");

    let test_list = &[
        "bitcoin", "take", "over", "the", "world", "maybe", "who", "knows", "perhaps",
    ];

    println!("{}", two_sort(test_list));
}

#[cfg(test)]
mod test {
    use super::two_sort;

    #[test]
    fn test_two_sort() {
        assert_eq!(
            two_sort(&[
                "bitcoin", "take", "over", "the", "world", "maybe", "who", "knows", "perhaps"
            ]),
            "b***i***t***c***o***i***n"
        );
        assert_eq!(
            two_sort(&[
                "turns", "out", "random", "test", "cases", "are", "easier", "than", "writing",
                "out", "basic", "ones"
            ]),
            "a***r***e"
        );
    }
}
