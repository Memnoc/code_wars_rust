// NOTE: write a function to remove
// all the exclamation marks from a given string
fn remove_exclamation_marks(input: &str) -> String {
    let input_string: String = input.to_string();
    input_string.chars().filter(|&c| c != '!').collect()
}

// NOTE: no significant drawbacks here, just a shorter
// and more elegant version
fn simpler_remove_exclamation_marks(input: &str) -> String {
    input.replace("!", "")
}

fn main() {
    println!("=== Remove Exclamation Marks ===");
    let result = remove_exclamation_marks("Hello World!");
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_exclamation_marks() {
        assert_eq!(remove_exclamation_marks("Hello World!"), "Hello World");
        assert_eq!(remove_exclamation_marks("Hello World!!!"), "Hello World");
        assert_eq!(remove_exclamation_marks("Hello!!! World!"), "Hello World");
        assert_eq!(remove_exclamation_marks("Hi!!"), "Hi");
        assert_eq!(remove_exclamation_marks(""), "");
    }
}
