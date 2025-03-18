use greetings::match_greeting;

mod greetings;

// HEADER: my solution
fn greet(language: &str) -> &str {
    match_greeting(language)
}

// TODO: create a local db
// the db contains entries for lanugages
// welcome() returns the corresponding greeting for
// the language
fn main() {
    println!("=== Welcome ===");
    println!("{}", greet("english"));
    println!("{}", greet(""));
}

#[cfg(test)]
mod test {
    use super::greet;

    #[test]
    fn test_greet() {
        assert_eq!(greet("english"), "Welcome");
        assert_eq!(greet("dutch"), "Welkom");
        assert_eq!(greet("IP_ADDRESS_INVALID"), "Welcome");
        assert_eq!(greet(""), "Welcome");
        assert_eq!(greet("swelsh"), "Welcome");
    }
}
