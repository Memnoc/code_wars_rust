fn find_smallest_int(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap_or(&i32::MAX)
}
// TODO: pretty easy but with the tricky bit of handling memory
fn main() {
    println!("=== Find smalles int ===");
    let test_array = [34, 15, 88, 2]; // expecting 2
    println!("{}", find_smallest_int(&test_array));
}

#[cfg(test)]
mod test {
    use super::find_smallest_int;

    #[test]
    fn test_find_smalles_int() {
        assert_eq!(find_smallest_int(&[34, 15, 88, 2]), 2);
        assert_eq!(find_smallest_int(&[34, -345, -1, 100]), -345);
    }
}
