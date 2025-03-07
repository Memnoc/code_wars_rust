fn points(games: &[String]) -> u32 {
    games
        .iter()
        .map(|game| {
            let scores: Vec<&str> = game.split(':').collect();

            let x: u32 = scores[0].parse().unwrap();
            let y: u32 = scores[1].parse().unwrap();

            if x > y {
                3
            } else {
                match x == y {
                    true => 1,
                    false => 0,
                }
            }
        })
        .sum()
}

// NOTE: Our football team has finished the championship.
// Our team's match results are recorded in a collection of strings.
// Each match is represented by a string in the format "x:y",
// where x is our team's score and y is our opponents score.
// For example: ["3:1", "2:2", "0:1", ...]
// NOTE: Points are awarded for each match as follows:
// if x > y: 3 points (win)
// if x < y: 0 points (loss)
// if x = y: 1 point (tie)
// HEADER: We need to write a function that takes this collection and returns
// the number of points our team (x) got in the championship by the rules given above.

fn main() {
    println!("=== Total Amount of Points ===");

    let games = vec!["1:0".to_string(), "2:2".to_string(), "0:1".to_string()];
    println!("Total points: {}", points(&games)); // Should output 4 (3 + 1 + 0)
}

#[cfg(test)]
mod tests {
    use super::points;
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    fn do_fixed_test(e: &[&str], expected: u32) {
        let a = &e.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        assert_eq!(points(a), expected, "{ERR_MSG} with games = {a:?}")
    }
    #[test]
    fn fixed_tests() {
        do_fixed_test(
            &[
                "1:0", "2:0", "3:0", "4:0", "2:1", "3:1", "4:1", "3:2", "4:2", "4:3",
            ],
            30,
        );
        do_fixed_test(
            &[
                "1:1", "2:2", "3:3", "4:4", "2:2", "3:3", "4:4", "3:3", "4:4", "4:4",
            ],
            10,
        );
        do_fixed_test(
            &[
                "0:1", "0:2", "0:3", "0:4", "1:2", "1:3", "1:4", "2:3", "2:4", "3:4",
            ],
            0,
        );
        do_fixed_test(
            &[
                "1:0", "2:0", "3:0", "4:0", "2:1", "1:3", "1:4", "2:3", "2:4", "3:4",
            ],
            15,
        );
        do_fixed_test(
            &[
                "1:0", "2:0", "3:0", "4:4", "2:2", "3:3", "1:4", "2:3", "2:4", "3:4",
            ],
            12,
        );
    }
}
