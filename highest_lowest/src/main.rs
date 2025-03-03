fn high_and_low(numbers: &str) -> String {
    let nums: Vec<i32> = numbers
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let max = nums.iter().max().unwrap();
    let min = nums.iter().min().unwrap();

    format!("{} {}", max, min)
}

// NOTE: given a string of space separated numbers,
// return the highest and lowest number.
fn main() {
    println!("=== Highest and Lowest ===");
    println!("{:?}", high_and_low("1 2 3 4 5"));
    println!("{:?}", high_and_low("1 2 -3 4 5"));
    println!("{:?}", high_and_low("1 9 3 4 -5"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_high_and_low() {
        assert_eq!(high_and_low("1 2 3 4 5"), "5 1");
        assert_eq!(high_and_low("1 2 -3 4 5"), "5 -3");
        assert_eq!(high_and_low("1 9 3 4 -5"), "9 -5");
        assert_eq!(high_and_low("8"), "8 8");
    }
}
