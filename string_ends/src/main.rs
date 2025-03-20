// HEADER: my solution
fn solution(word: &str, ending: &str) -> bool {
    if ending.len() > word.len() {
        return false;
    }

    word.ends_with(ending)
}

// HEADER: another solution of mine but more idiomatic
fn alternate_solution(word: &str, ending: &str) -> bool {
    word.ends_with(ending)
}

// TODO: Complete the solution so that it returns true
// if the first argument passed in ends with the 2nd argument
fn main() {
    println!("=== String ends with? ===");
    solution("abc", "bc"); // true
    solution("abcd", "d"); // false
                           //

    alternate_solution("abc", "bc"); // true
    alternate_solution("abcd", "d"); // false
}

#[cfg(test)]
mod test {
    use super::solution;

    #[test]
    fn test_solution() {
        assert!(solution("abc", "c"));
        assert!(!solution("strawberry", "banana"));
    }
}
