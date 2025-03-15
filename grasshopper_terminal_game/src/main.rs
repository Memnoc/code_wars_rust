use rand::Rng;

// HEADER: my solution
fn move_hero(position: u32, roll: u32) -> u32 {
    position + roll * 2
}

// TODO: for an hypothetical game:
// create a function that takes the current position of the hero
// and the roll (1-6) and return the new position
fn main() {
    println!("=== Grasshopper Terminal game move function ===");
    let mut random = rand::rng();
    let current_position = 0;
    let dice_roll = random.random_range(1..=6) * 2;
    let new_position = move_hero(current_position, dice_roll);

    println!("Current position: {}", current_position);
    println!("Dice roll: {}", dice_roll);
    println!("New position: {}", new_position);
}

#[cfg(test)]
mod test {
    use super::move_hero;
    #[test]
    fn test_move_hero() {
        assert_eq!(move_hero(0, 4), 8);
        assert_eq!(move_hero(4, 1), 6);
    }
}
