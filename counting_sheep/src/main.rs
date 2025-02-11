// NOTE: write a function that counts the occurrences of a boolean = true
fn count_sheep(values: &[bool]) -> u8 {
    values.iter().filter(|&&x| x).count().try_into().unwrap()
}
fn main() {
    println!("== Counting Sheep ==");
    let total_sheep: [bool; 24] = [
        true, true, true, false, true, true, true, true, true, false, true, false, true, false,
        false, true, true, true, true, true, false, false, true, true,
    ];

    count_sheep(&total_sheep);
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
    }
}
