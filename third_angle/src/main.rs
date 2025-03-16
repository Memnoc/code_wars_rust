// HEADER: my solution
fn other_angle(a: u32, b: u32) -> u32 {
    180 - a - b
}
fn main() {
    println!("=== Third angle of a triangle ===");
    other_angle(30, 60);
}

#[cfg(test)]
mod test {
    use super::other_angle;

    #[test]
    fn test_other_angle() {
        assert_eq!(other_angle(30, 60), 90);
        assert_eq!(other_angle(60, 60), 60);
        assert_eq!(other_angle(43, 78), 59);
        assert_eq!(other_angle(30, 60), 90);
        assert_eq!(other_angle(10, 20), 150);
    }
}
