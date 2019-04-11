use log::info;
use rand::{rngs::ThreadRng, Rng};
use regex::Regex;
use serde::Serialize;

#[derive(Debug, PartialEq)]
struct RollInstructions {
    num: i32,
    dice: i32,
}

#[derive(Serialize, Debug)]
pub struct DiceResult {
    pub dice: i32,
    pub value: i32,
}

#[derive(Serialize, Debug)]
pub struct RollResult {
    pub instruction: String,
    pub rolls: Vec<DiceResult>,
    pub total: i32,
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

fn gen_roll(rng: &mut ThreadRng, dice: i32) -> DiceResult {
    let roll = rng.gen_range(1, dice + 1);
    info!("Dice: {}, Roll: {}", dice, roll);
    DiceResult { dice, value: roll }
}

pub fn roll(cmd: &str) -> Result<RollResult, &str> {
    let mut rng = rand::thread_rng();
    let roll_instructions = parse_roll(cmd)?;
    let mut total = 0;
    let mut rolls = Vec::new();
    for instruction in roll_instructions {
        if instruction.num < 1 {
            return Err("You have to roll something!");
        } else if instruction.num > 99 {
            return Err("Are you a god in this game?! Roll a more reasonable number of dice!");
        }
        for _ in 0..instruction.num {
            let roll = gen_roll(&mut rng, instruction.dice);
            total += roll.value;
            rolls.push(roll);
        }
    }
    Ok(RollResult {
        instruction: cmd.to_string(),
        rolls,
        total,
    })
}

#[cfg(test)]
mod tests {
    // All the possible D&D dice
    const DICE_VALUES: [i32; 7] = [4, 6, 8, 10, 12, 20, 100];

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

        for d in DICE_VALUES.iter() {
            let mut occurrences: HashMap<i32, i32> = HashMap::new();
            // Try and get a sample that will have an occurrence for every value
            for _ in 0..d * d {
                let roll = gen_roll(&mut rng, *d);
                let count = occurrences.entry(roll.value).or_insert(0);
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
        assert!(roll.total >= 1);
        assert!(roll.total <= 8);
    }

    #[test]
    fn test_roll_multiple_dice() {
        let roll = roll("3d6").unwrap();
        assert!(roll.total >= 3);
        assert!(roll.total <= 18);
    }

    #[test]
    #[should_panic]
    fn test_roll_fail() {
        roll("3e6").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_roll_too_few() {
        roll("0d6").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_roll_too_many() {
        roll("100e6").unwrap();
    }
}
