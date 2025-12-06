use anyhow::Result;
use log::{debug, trace};
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
enum Direction {
    R,
    L,
}

#[derive(Copy, Clone)]
struct Rotation {
    direction: Direction,
    value: usize,
}

impl FromStr for Rotation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let direction = match s.chars().next() {
            Some('L') => Direction::L,
            Some('R') => Direction::R,
            _ => anyhow::bail!("Invalid direction: expected 'L' or 'R'"),
        };

        let value = s[1..].parse::<usize>()?;

        Ok(Rotation { direction, value })
    }
}

impl Debug for Rotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{}", self.direction, self.value)
    }
}

#[derive(Debug)]
struct Dial(usize);

impl Dial {
    const SIZE: usize = 100;

    fn new() -> Self {
        Self(50)
    }

    fn rotate(self, rotation: Rotation) -> Self {
        trace!("Rotating {self:?} {rotation:?}");

        let magnitude = rotation.value % Self::SIZE;

        let value = match rotation.direction {
            Direction::L => (self.0 + Self::SIZE - magnitude) % Self::SIZE,
            Direction::R => (self.0 + magnitude) % Self::SIZE,
        };

        let dial = Self(value);

        debug!("Rotated to {dial:?}");
        dial
    }
}

fn password(input: Vec<String>) -> usize {
    let rotations = input
        .into_iter()
        .filter_map(|l| Rotation::from_str(l.as_str()).ok())
        .collect::<Vec<Rotation>>();

    let mut password = 0;

    let mut dial = Dial::new();
    for rotation in rotations {
        dial = dial.rotate(rotation);
        if dial.0 == 0 {
            password += 1;
        }
    }

    password
}

fn main() -> Result<()> {
    let password = password(aoc_util::init()?);
    println!("{password}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotation_parsing() -> Result<()> {
        let rotation_l38 = "L38".parse::<Rotation>()?;
        assert!(matches!(rotation_l38.direction, Direction::L));
        assert_eq!(rotation_l38.value, 38);

        let rotation_r14 = "R14".parse::<Rotation>()?;
        assert!(matches!(rotation_r14.direction, Direction::R));
        assert_eq!(rotation_r14.value, 14);

        Ok(())
    }

    #[test]
    fn part_1_example_rotations() -> Result<()> {
        let input = aoc_util::init_test()?;
        let rotations = input
            .into_iter()
            .filter_map(|l| Rotation::from_str(l.as_str()).ok())
            .collect::<Vec<Rotation>>();

        assert_eq!(10, rotations.len());

        let mut dial = Dial::new();
        assert_eq!(50, dial.0);

        let expected = vec![82, 52, 0, 95, 55, 0, 99, 0, 14, 32];

        for (expected, rotation) in expected.into_iter().zip(rotations.into_iter()) {
            dial = dial.rotate(rotation);
            println!("{dial:?}");
            assert_eq!(expected, dial.0);
        }

        Ok(())
    }

    #[test]
    fn part_1_example() -> Result<()> {
        assert_eq!(3, password(aoc_util::init_test()?));

        Ok(())
    }
}
