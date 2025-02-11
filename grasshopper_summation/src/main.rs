fn summation(n: i32) -> i32 {
    n * (n + 1) / 2
}

fn main() {
    println!("=== Grasshopper Summation ===");
    let num_one = 2;
    let num_two = 8;

    summation(num_one);
    summation(num_two);
    println!("{}: ", summation(4))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summation() {
        assert_eq!(summation(1), 1);
        assert_eq!(summation(8), 36);
        assert_eq!(summation(22), 253);
        assert_eq!(summation(100), 5050);
        assert_eq!(summation(213), 22791);
    }
}
