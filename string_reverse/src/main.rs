// NOTE: Reverse the string
// 'world'  =>  'dlrow'
// 'word'   =>  'drow'

fn reverse_string(input: &str) -> String {
    let reversed = input.chars().rev().collect::<String>();
    reversed
}

fn main() {
    println!("== String Reverse ==");
    let basic_reversal = "world";
    let empty_reversal = "";
    let single_reversal = "w";
    let with_spaces = "hello world";
    let special_reversal = "hello!@#";

    let input_one = reverse_string(basic_reversal);
    println!("Original: | {}, | Reversed: {}", basic_reversal, input_one);
    let input_two = reverse_string(empty_reversal);
    println!("Original: | {}, | Reversed: {}", empty_reversal, input_two);
    let input_three = reverse_string(single_reversal);
    println!("Original: {}, | Reversed: {}", single_reversal, input_three);
    let input_four = reverse_string(with_spaces);
    println!("Original: {}, | Reversed: {}", with_spaces, input_four);
    let input_five = reverse_string(special_reversal);
    println!("Original: {}, | Reversed: {}", special_reversal, input_five);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string() {
        // Basic reversal
        assert_eq!(reverse_string("hello"), "olleh");

        // Empty string
        assert_eq!(reverse_string(""), (""));

        // Single character
        assert_eq!(reverse_string("a"), ("a"));

        // With spaces
        assert_eq!(reverse_string("hello world"), ("dlrow olleh"));

        // With special characters
        assert_eq!(reverse_string("hello!@#"), "#@!olleh");
    }
}
