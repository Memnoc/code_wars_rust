// NOTE: create a function which answers the question
// "Are you playing banjo?"
// If your name starts with "R" or lower case "r" -> you are playing the banjo!
// fn are_you_playing_banjo(name: &str) -> String {
//     format!(
//         "{} {} banjo",
//         name,
//         if name.starts_with(|c: char| c.eq_ignore_ascii_case(&'r')) {
//             "plays"
//         } else {
//             "does not play"
//         }
//     )
// }

// A better, more idiomatic version
// fn are_you_playing_banjo(name: &str) -> String {
//     match &name[0..1] {
//         "R" | "r" => format!("{} plays banjo", name),
//         _ => format!("{} does not play banjo", name),
//     }
// }

// Safest and reasonably fast version
fn are_you_playing_banjo(name: &str) -> String {
    match name.chars().next() {
        Some(c) if c.eq_ignore_ascii_case(&'r') => format!("{} plays banjo", name),
        Some(_) => format!("{} does not play banjo", name),
        None => String::from("Name cannot be empty"),
    }
}

fn main() {
    println!("=== Playing Banjo ===");
    let name_one = "Rikke";
    let name_two = "bravo";
    let name_three = "rolf";

    are_you_playing_banjo(name_one);
    are_you_playing_banjo(name_two);
    are_you_playing_banjo(name_three);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_you_playing_banjo() {
        assert_eq!(
            are_you_playing_banjo("Martin"),
            "Martin does not play banjo"
        );
        assert_eq!(are_you_playing_banjo("Rikke"), "Rikke plays banjo");
        assert_eq!(are_you_playing_banjo("bravo"), "bravo does not play banjo");
        assert_eq!(are_you_playing_banjo("rolf"), "rolf plays banjo");
    }
}
