// NOTE: Given an array of integers as strings and numbers,
// return the sum of the array values as if all were numbers.
// Return your answer as a number.

use either::Either;

fn sum_mix(arr: &[Either<i32, String>]) -> i32 {
    todo!()
}
fn main() {
    println!("=== Sum mixed array ===");
}

#[cfg(test)]
mod test {
    use super::sum_mix;
    use either::Either;

    fn dotest(arr: &[Either<i32, String>], expected: i32) {
        let actual = sum_mix(arr);
        assert!(
            actual == expected,
            "With arr = {arr:?}\nExpected {expected} but got {actual}"
        );
    }
}
