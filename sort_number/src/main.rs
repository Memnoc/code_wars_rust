// HEADER: my solution
fn sort_numbers(arr: &Vec<i32>) -> Vec<i32> {
    todo!()
}

// TODO: write a function to sort an array of numbers
// if the function passes and empty or nil array it must
// return an empty array
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::sort_numbers;

    #[test]
    fn test_sort_numbers() {
        assert_eq!(sort_numbers(&vec![1, 2, 3, 10, 5]), vec![1, 2, 3, 5, 10]);
        assert_eq!(sort_numbers(&vec![]), vec![]);
        assert_eq!(sort_numbers(&vec![20, 2, 10]), vec![2, 10, 20]);
        assert_eq!(sort_numbers(&vec![2, 20, 10]), vec![2, 10, 20]);
    }
}
