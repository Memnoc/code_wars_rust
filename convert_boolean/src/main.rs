fn bool_to_test(value: bool) -> &'static str {
    match value {
        true => "Yes",
        false => "No",
    }
}
fn main() {
    println!("=== Bool to String ===");
    let is_good = true;
    let is_false = false;

    println!("{}", bool_to_test(is_good));
    println!("{}", bool_to_test(is_false));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn do_test() {
        assert_eq!(bool_to_test(true), "Yes");
        assert_eq!(bool_to_test(false), "No");
    }
}
