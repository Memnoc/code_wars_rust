// HEADER: my solution

fn check_for_factor(base: i32, factor: i32) -> bool {
    base % factor == 0
}

// TODO: check if factor is a factor of base
// return true if it is a factor or else false
fn main() {
    println!("=== Grasshopper - check for factor ===");
}

#[cfg(test)]
mod test {
    use super::check_for_factor;

    #[test]
    fn test_check_for_factor() {
        assert!(check_for_factor(10, 2), "10 is divisible by 2");
        assert!(check_for_factor(63, 7), "63 is divisible by 7");
        assert!(check_for_factor(2450, 5), "2450 is divisible by 5");
        assert!(check_for_factor(24612, 3), "24612 is divisible by 3");
        assert!(!check_for_factor(9, 2), "9 is not divisible by 2");
        assert!(!check_for_factor(653, 7), "653 is not divisible by 7");
        assert!(!check_for_factor(2453, 5), "2453 is not divisible by 5");
        assert!(!check_for_factor(24617, 3), "24617 is not divisible by 3");
    }
}
