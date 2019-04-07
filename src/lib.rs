use log::info;
use rand::{rngs::ThreadRng, Rng};
use regex::Regex;

#[derive(Debug, PartialEq)]
struct RollInstructions {
    num: u32,
    dice: u32,
}

fn parse_roll(cmd: &str) -> Result<Vec<RollInstructions>, &str> {
    let re = Regex::new(r"(?P<num>\d+)d(?P<dice>\d+)").unwrap();
    if re.is_match(cmd) {
        let rolls: Vec<RollInstructions> = re
            .captures_iter(cmd)
            .map(|cap| RollInstructions {
                num: cap["num"].parse().unwrap(),
                dice: cap["dice"].parse().unwrap(),
            })
            .collect();
        Ok(rolls)
    } else {
        Err("Invalid format. Try again with something like 1d20 or 3d6.")
    }
}

fn gen_roll(rng: &mut ThreadRng, d: u32) -> u32 {
    let roll = rng.gen_range(1, d + 1);
    info!("Dice: {}, Roll: {}", d, roll);
    roll
}

pub fn roll(cmd: &str) -> Result<u32, &str> {
    let mut rng = rand::thread_rng();
    let roll_instructions = parse_roll(cmd)?;
    let mut sum: u32 = 0;
    for roll in roll_instructions {
        sum += (0..roll.num).fold(0, |a, _| a + gen_roll(&mut rng, roll.dice));
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_parse_roll_single_dice() {
        let roll = parse_roll("1d8").unwrap();
        assert_eq!(roll, [RollInstructions { num: 1, dice: 8 }]);
    }

    #[test]
    fn test_parse_roll_multiple_dice() {
        let roll = parse_roll("3d6").unwrap();
        assert_eq!(roll, [RollInstructions { num: 3, dice: 6 }]);
    }

    #[test]
    #[should_panic]
    fn test_parse_roll_fail() {
        parse_roll("3e6").unwrap();
    }

    #[test]
    fn test_gen_roll() {
        let mut rng = rand::thread_rng();
        // All the possible D&D dice
        let dice_values: [u32; 7] = [4, 6, 8, 10, 12, 20, 100];

        for d in dice_values.iter() {
            let mut occurrences: HashMap<u32, u32> = HashMap::new();
            // Try and get a sample that will have an occurrence for every value
            for _ in 0..d * d {
                let roll: u32 = gen_roll(&mut rng, *d);
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
