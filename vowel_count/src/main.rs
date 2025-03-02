use std::usize;

// NOTE: Return the number (count)
// of vowels in a given string

// HEADER: My solution
fn get_count(string: &str) -> usize {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut vowels_count: usize = 0;

    for c in string.chars() {
        if vowels.contains(&c.to_lowercase().next().unwrap()) {
            vowels_count += 1;
        }
    }
    vowels_count
}

// better solution
fn better_get_count(string: &str) -> usize {
    string.chars().filter(|&c| "aeiou".contains(c)).count()
}

fn main() {
    println!("=== Vowel Count ===");
    let test_word = "abracadabra";
    println!("{:?}", get_count(test_word));
    println!("{:?}", better_get_count(test_word));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_count() {
        assert_eq!(get_count("abracadabra"), 5);
    }
}
