use greetings::match_greeting;

mod greetings;

// HEADER: my solution
fn greet(language: &str) -> &str {
    match_greeting(language)
}

// HEADER: alternative solution is embrassingly easier than mine
fn alternate_greet(language: &str) -> &str {
    match language {
        "english" => "Welcome",
        "czech" => "Vitejte",
        "danish" => "Velkomst",
        "dutch" => "Welkom",
        "estonian" => "Tere tulemast",
        "finnish" => "Tervetuloa",
        "flemish" => "Welgekomen",
        "french" => "Bienvenue",
        "german" => "Willkommen",
        "irish" => "Failte",
        "italian" => "Benvenuto",
        "latvian" => "Gaidits",
        "lithuanian" => "Laukiamas",
        "polish" => "Witamy",
        "spanish" => "Bienvenido",
        "swedish" => "Valkommen",
        "welsh" => "Croeso",
        _ => "Welcome",
    }
}

// TODO: create a local db
// the db contains entries for lanugages
// welcome() returns the corresponding greeting for
// the language
fn main() {
    println!("=== Welcome ===");
    println!("{}", greet("english"));
    println!("{}", greet(""));

    println!("{}", alternate_greet("english"));
    println!("{}", alternate_greet("english"));
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
