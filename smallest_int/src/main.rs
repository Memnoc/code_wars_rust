fn find_smallest_int(arr: &[i32]) -> i32 {
    todo!()
}
fn main() {
    println!("=== Find smalles int ===");
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
