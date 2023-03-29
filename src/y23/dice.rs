use std::{fmt::Display, str::FromStr};

use rand::Rng;

#[derive(PartialEq, Eq, Debug)]
struct Dice {
    num: usize,
    sides: usize,
}

impl Display for Dice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}d{}", self.num, self.sides)
    }
}

impl Dice {
    pub fn roll(&self) -> usize {
        let mut rng = rand::thread_rng();

        self.roll_with_rng(&mut rng)
    }

    pub fn roll_with_rng(&self, rng: &mut impl Rng) -> usize {
        let result = (0..self.num).map(|_| rng.gen_range(1..=self.sides)).sum();

        println!("{self} rolled: {result}");

        result
    }
}

impl FromStr for Dice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (num, sides) = s
            .split_once('d')
            .ok_or_else(|| format!("malformed string: {s}"))?;

        let d = Dice {
            num: num
                .parse::<usize>()
                .map_err(|err| format!("failed parsing number: {err}"))?,
            sides: sides
                .parse::<usize>()
                .map_err(|err| format!("failed parsing sides: {err}"))?,
        };

        Ok(d)
    }
}

pub fn roll_dice(input: &str) -> usize {
    input
        .split('+')
        .map(|d| d.parse::<Dice>().unwrap())
        .map(|d| d.roll())
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    use rand::rngs::mock::StepRng;
    use test_case::test_case;

    #[test_case("1d8", 1, 8)]
    #[test_case("5d10", 5, 10)]
    #[test_case("3d4", 3, 4)]
    fn test_parse(input: &str, expected_num: usize, expected_sides: usize) {
        let dice: Dice = input.parse().unwrap();

        assert_eq!(
            dice,
            Dice {
                num: expected_num,
                sides: expected_sides
            }
        )
    }

    #[test_case("1d8", 1)]
    #[test_case("4d4", 4)]
    #[test_case("10d9", 10)]
    fn test_roll(input: &str, expected: usize) {
        let mut rng = StepRng::new(1, 0);

        let d: Dice = input.parse().unwrap();

        assert_eq!(d.roll_with_rng(&mut rng), expected)
    }
}
