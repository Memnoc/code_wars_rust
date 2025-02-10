// NOTE: write a function that counts the occurrences of a boolean = true
fn count_sheep(values: &[bool]) -> usize {
    todo!()
}
fn main() {
    println!("== Counting Sheep ==");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_sheep() {
        assert_eq!(count_sheep(&[true, false, true, true]), 3);
        assert_eq!(count_sheep(&[]), 0);
        assert_eq!(count_sheep(&[false, false, false]), 0);
        assert_eq!(count_sheep(&[true; 5]), 5);
        todo!()
    }
}
