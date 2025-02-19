// NOTE: Complete the function so that it
// finds the average of the three scores passed to it
// and returns the letter value associated with that grade.
//Numerical Score 	Letter Grade
// 90 <= score <= 100 	'A'
// 80 <= score < 90 	'B'
// 70 <= score < 80 	'C'
// 60 <= score < 70 	'D'
// 0 <= score < 60 	'F'

// Tested values are all between 0 and 100.
// Theres is no need to check for negative values or values greater than 100.

fn get_grade(s1: u16, s2: u16, s3: u16) -> char {
    let average_score = (s1 + s2 + s3) / 3;
    match average_score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    }
}

fn main() {
    println!("=== Grasshopper: Grade Book ===");
    let score_one = 82;
    let score_two = 85;
    let score_three = 87;

    let average_score = get_grade(score_one, score_two, score_three);
    println!("average: {}", average_score);
}

#[cfg(test)]
mod tests {
    use super::get_grade;

    fn dotest(s1: u16, s2: u16, s3: u16, expected: char) {
        assert_eq!(
            get_grade(s1, s2, s3),
            expected,
            "Scores [{}, {}, {}] should result in grade '{}', but got '{}'",
            s1,
            s2,
            s3,
            expected,
            get_grade(s1, s2, s3)
        );
    }

    #[test]
    fn test_perfect_scores() {
        dotest(100, 100, 100, 'A');
    }

    #[test]
    fn test_excellent_scores() {
        dotest(95, 90, 93, 'A');
    }

    #[test]
    fn test_average_scores() {
        dotest(82, 85, 87, 'B');
    }
}
