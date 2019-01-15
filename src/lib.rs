use rand::Rng;

pub fn roll(d: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1, d + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_roll() {
        // All the possible D&D dice
        let dice_values: [u32; 6] = [4, 6, 8, 10, 12, 20];

        for d in dice_values.iter() {
            let mut occurrences: HashMap<u32, u32> = HashMap::new();
            // Try and get a sample that will have an occurrence for every value
            for _ in 0..d * d {
                let roll: u32 = roll(*d);
                let count = occurrences.entry(roll).or_insert(0);
                *count += 1;
            }

            // Assert that all values for 1 through d have at least one roll
            for i in 1..=*d {
                assert!(occurrences[&i] > 0)
            }
        }
    }
}
