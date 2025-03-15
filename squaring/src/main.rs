// HEADER: my simple solution
fn square(number: i32) -> i32 {
    number * number
    // number.pow(2) // also valid
}

// TODO: write a function that takes
// a number and returns its square
fn main() {
    println!("=== Squaring an argument ===");
    println!("{:?}", square(2));
    println!("{:?}", square(4));
    println!("{:?}", square(5));
}

#[cfg(test)]
mod test {
    use super::square;

    #[test]
    fn test_square() {
        assert_eq!(square(1), 1);
        assert_eq!(square(2), 4);
        assert_eq!(square(3), 9);
        assert_eq!(square(4), 16);
        assert_eq!(square(5), 25);
    }
}
