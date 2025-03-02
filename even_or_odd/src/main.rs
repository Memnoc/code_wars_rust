// NOTE: Create a function that takes an integer
// as an argument and returns "Even"
// for even numbers or "Odd" for odd numbers.

// HEADER: My solution
fn even_or_odd(input: i32) -> &'static str {
    match input % 2 == 0 {
        true => "Even",
        false => "Odd",
    }
}

// HEADER: very cool use of cast and tuples
fn even_or_odd_alternative(i: i32) -> &'static str {
    ["Even", "Odd"][i as usize % 2]
}
fn main() {
    println!("=== Even or Odd ===");
    println!("{}", even_or_odd(4));
    println!("{}", even_or_odd(1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_or_odd() {
        assert_eq!(even_or_odd(4), "Even");
        assert_eq!(even_or_odd(1), "Odd");
    }
}
