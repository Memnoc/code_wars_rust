use std::collections::HashMap;

pub fn create_greetings() -> HashMap<&'static str, &'static str> {
    let mut greetings = HashMap::new();
    for (language, greeting) in [
        ("english", "Welcome"),
        ("czech", "Vitejte"),
        ("danish", "Velkomst"),
        ("dutch", "Welkom"),
        ("estonian", "Tere tulemast"),
        ("finnish", "Tervetuloa"),
        ("flemish", "Welgekomen"),
        ("french", "Bienvenue"),
        ("german", "Willkommen"),
        ("irish", "Failte"),
        ("italian", "Benvenuto"),
        ("latvian", "Gaidits"),
        ("lithuanian", "Laukiamas"),
        ("polish", "Witamy"),
        ("spanish", "Bienvenido"),
        ("swedish", "Valkommen"),
        ("welsh", "Croeso"),
    ] {
        greetings.insert(language, greeting);
    }
    greetings
}

pub fn match_greeting(language: &str) -> &'static str {
    create_greetings()
        .get(language)
        .copied()
        .unwrap_or("Welcome")
}
