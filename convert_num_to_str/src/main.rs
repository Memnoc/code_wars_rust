// NOTE: https://www.codewars.com/kata/5265326f5fda8eb1160004c8/rust
// 123  --> "123"
// 999  --> "999"
// -100 --> "-100"

fn number_to_string(n: i32) -> String {
    n.to_string()
}

fn main() {
    println!("=== Convert a Number to a String ===");
    let conversion = number_to_string(67);
    println!("{}", conversion);
}

#[cfg(test)]
mod tests {
    use super::number_to_string;
    fn dotest(n: i32, expected: &str) {
        let actual = number_to_string(n);
        assert!(
            actual == expected,
            "With n = {n}\nExpected \"{expected}\" but got \"{actual}\""
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(67, "67");
        dotest(79585, "79585");
        dotest(1 + 2, "3");
        dotest(1 - 2, "-1");
        dotest(0, "0");
    }
}
