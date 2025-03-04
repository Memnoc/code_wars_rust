fn number(bus_stops: &[(i32, i32)]) -> i32 {
    todo!()
}

fn main() {
    println!("=== Number of People in the Bus ===");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_number() {
        assert_eq!(number(&[(10, 0), (3, 5), (5, 8)]), 5);
        assert_eq!(
            number(&[(3, 0), (9, 1), (4, 10), (12, 2), (6, 1), (7, 10)]),
            17
        );
        assert_eq!(
            number(&[(3, 0), (9, 1), (4, 8), (12, 2), (6, 1), (7, 8)]),
            21
        );
    }
}
