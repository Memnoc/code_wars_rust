fn is_square(n: i64) -> bool {
    if n < 0 {
        return false;
    }

    let root = (n as f64).sqrt() as i64;
    root * root == n
}

// TODO: given an integral number, determine if it is a square number
// We are gonna pause this one until we find it again
fn main() {
    println!("=== Square of Squares ===");

    print!("{}", is_square(16));
    print!("{}", is_square(25));
    print!("{}", is_square(26));
    print!("{}", is_square(-4));
}

#[cfg(test)]
mod test {
    use super::is_square;

    #[test]
    fn test_is_square() {
        assert!(is_square(0));
        assert!(is_square(1));
        assert!(is_square(4));
    }
}
