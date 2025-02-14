// NOTE: find the max and min values of a list
// [4,6,2,1,9,63,-134,566]         -> max = 566, min = -134
// [-52, 56, 30, 29, -54, 0, -110] -> min = -110, max = 56
// [42, 54, 65, 87, 0]             -> min = 0, max = 87
// [5]

// NOTE: good, readable and performant but somewhat redundant in some parts
// like not using tuples
fn maximum(arr: &[i32]) -> Option<i32> {
    if arr.is_empty() {
        return None;
    }
    let mut min_num = arr[0];

    for &num in arr {
        if num > min_num {
            min_num = num;
        }
    }
    Some(min_num)
}

fn minimum(arr: &[i32]) -> Option<i32> {
    if arr.is_empty() {
        return None;
    }
    let mut min_num = arr[0];

    for &num in arr {
        if num <= min_num {
            min_num = num
        }
    }
    Some(min_num)
}

// NOTE: The best!
fn min_max(arr: &[i32]) -> Option<(i32, i32)> {
    if arr.is_empty() {
        return None;
    }

    let mut min = arr[0];
    let mut max = arr[0];

    for &num in arr {
        if num < min {
            min = num;
        }
        if num > max {
            max = num;
        }
    }

    Some((min, max))
}

// NOTE: a very idiomatic approach and super short
// not too good
fn code_wars_minimum(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}

fn code_wars_maximum(arr: &[i32]) -> i32 {
    *arr.iter().max().unwrap()
}

// NOTE: best idtiomatic approach IMO
fn idiomatic_minimum(arr: &[i32]) -> i32 {
    match arr.iter().min() {
        Some(&min) => min,
        None => 0,
    }
}

fn idiomatic_maximum(arr: &[i32]) -> i32 {
    match arr.iter().max() {
        Some(&max) => max,
        None => 0,
    }
}

fn main() {
    println!("=== Max and Min ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(arr: &[i32], expected_min: i32, expected_max: i32) {
        assert_eq!(
            minimum(arr),
            Some(expected_min),
            "{ERR_MSG} with function minimum and arr = {arr:?}"
        );
        assert_eq!(
            maximum(arr),
            Some(expected_max),
            "{ERR_MSG} with function maximum and arr = {arr:?}"
        );
    }

    #[test]
    fn fixed_tests() {
        dotest(&[-52, 56, 30, 29, -54, 0, -110], -110, 56);
        dotest(&[42, 54, 65, 87, 0], 0, 87);
        dotest(&[1, 2, 3, 4, 5, 10], 1, 10);
        dotest(
            &[
                -1, -2, -3, -4, -5, -10, 534, 43, 2, 1, 3, 4, 5, 5, 443, 443, 555, 555,
            ],
            -10,
            555,
        );
        dotest(&[9], 9, 9);
        dotest(&[4, 6, 2, 1, 9, 63, -134, 566], -134, 566);
    }
}
