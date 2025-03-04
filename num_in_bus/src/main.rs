// HEADER: My solution
fn number(bus_stops: &[(i32, i32)]) -> i32 {
    let people_on: i32 = bus_stops.iter().map(|&(on, _)| on).sum();
    let people_off: i32 = bus_stops.iter().map(|&(_, off)| off).sum();

    people_on - people_off
}

// NOTE: this is brilliant code
fn number_alternative_one(bus_stops: &[(i32, i32)]) -> i32 {
    bus_stops.iter().map(|(a, b)| a - b).sum()
}

// NOTE: another fantastic soltion
fn number_alternative_two(bus_stops: &[(i32, i32)]) -> i32 {
    bus_stops.iter().fold(0, |acc, x| acc + x.0 - x.1)
}

// INFO: There is a bus moving in the city which takes and drops some people at each bus stop.
// You are provided with a list (or array) of integer pairs.
// Elements of each pair represent the number of people that get on the bus (the first item)
// and the number of people that get off the bus (the second item) at a bus stop.
// ***********************************************************
// HEADER: Your task is to return the number of people who are still on the bus
// after the last bus stop (after the last array).
// Even though it is the last bus stop, the bus might not be empty
// and some people might still be inside the bus,
// they are probably sleeping there :D"
//
// NOTE: easiset way would be to sum the first element of the array
// then the second, then subtract them
fn main() {
    println!("=== Number of People in the Bus ===");
    println!("{}", number(&[(10, 0), (3, 5), (5, 8)]));
    println!(
        "{}",
        number(&[(3, 0), (9, 1), (4, 10), (12, 2), (6, 1), (7, 10)])
    );
    println!(
        "{}",
        number(&[(3, 0), (9, 1), (4, 8), (12, 2), (6, 1), (7, 8)])
    );
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
