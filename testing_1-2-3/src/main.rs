// HEADER: my solution
fn number(lines: &[&str]) -> Vec<String> {
    let new_line = lines
        .iter()
        .enumerate()
        .map(|(i, l)| format!("{}: {}", i + 1, l))
        .collect();

    new_line
}

// HEADER: alternate syntax
// zip() zips two iterators into one
// interesting method but I am not sure
// there are any upside to enumerate()
fn alternate_number(lines: &[&str]) -> Vec<String> {
    lines
        .iter()
        .zip(1..)
        .map(|(val, index)| format!("{index}: {val}"))
        .collect()
}

// TODO: implement the line numbers for a text editor
fn main() {
    println!("Testing 1-2-3");
    let test = &["a", "b", "c"];
    println!("{:?}", number(test));
}

#[cfg(test)]
mod test {
    use super::number;

    fn dotest(arr: &[&str], expected: &[&str]) {
        let actual = number(arr);
        assert!(
            actual == expected,
            "With lines= {arr:?}\nExpected {expected:?}\nBut got {actual:?}"
        )
    }

    #[test]
    fn sample_tests() {
        dotest(&[], &[]);
        dotest(&["a", "b", "c"], &["1: a", "2: b", "3: c"]);
        dotest(&["", "", ""], &["1: ", "2: ", "3: "]);
        dotest(&["", "b", "", ""], &["1: ", "2: b", "3: ", "4: "]);
    }
}
