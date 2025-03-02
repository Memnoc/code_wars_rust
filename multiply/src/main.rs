// NOTE: fix the code that does not execute well
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
fn main() {
    println!("=== Multiply ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(4, 4), 16);
    }
}
