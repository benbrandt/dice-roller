use log::info;
use rand::Rng;
use regex::Regex;

fn gen_roll(d: u32) -> u32 {
    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(1, d + 1);
    info!("Dice: {}, Roll: {}", d, roll);
    roll
}

pub fn roll(dice: &str) -> Result<u32, &str> {
    let re = Regex::new(r"^(?P<num>\d+)d(?P<d>\d+)$").unwrap();
    if re.is_match(dice) {
        let mut sum: u32 = 0;
        for cap in re.captures_iter(dice) {
            sum += (0..cap["num"].parse().unwrap())
                .fold(0, |a, _| a + gen_roll(cap["d"].parse().unwrap()));
        }
        Ok(sum)
    } else {
        Err("Invalid format. Try again with something like 1d20 or 3d6.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_gen_roll() {
        // All the possible D&D dice
        let dice_values: [u32; 6] = [4, 6, 8, 10, 12, 20];

        for d in dice_values.iter() {
            let mut occurrences: HashMap<u32, u32> = HashMap::new();
            // Try and get a sample that will have an occurrence for every value
            for _ in 0..d * d {
                let roll: u32 = gen_roll(*d);
                let count = occurrences.entry(roll).or_insert(0);
                *count += 1;
            }

            // Assert that all values for 1 through d have at least one roll
            for i in 1..=*d {
                assert!(occurrences[&i] > 0)
            }
        }
    }

    #[test]
    fn test_roll_single_dice() {
        let roll = roll("1d8").unwrap();
        assert!(roll >= 1);
        assert!(roll <= 8);
    }

    #[test]
    fn test_roll_multiple_dice() {
        let roll = roll("3d6").unwrap();
        assert!(roll >= 3);
        assert!(roll <= 18);
    }

    #[test]
    #[should_panic]
    fn test_roll_fail() {
        roll("3e6").unwrap();
    }
}
