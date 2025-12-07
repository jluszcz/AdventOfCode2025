use anyhow::{bail, Result};
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug)]
struct Ranges(Vec<Range>);

impl TryFrom<Vec<String>> for Ranges {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> std::result::Result<Self, Self::Error> {
        if value.len() != 1 {
            bail!("Expected a single range, got {}", value.len());
        }

        let mut ranges = Vec::new();
        for l in value {
            for r in l.split(',') {
                ranges.push(Range::from_str(r)?);
            }
        }
        Ok(Self(ranges))
    }
}

#[derive(Debug)]
struct Range(usize, usize);

impl FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').unwrap();
        Ok(Self(start.parse::<usize>()?, end.parse::<usize>()?))
    }
}

impl Range {
    fn invalid_ids(&self) -> Vec<usize> {
        let mut invalid_ids = Vec::new();

        for i in self.0..=self.1 {
            // Count digits
            let num_digits = if i == 0 {
                1
            } else {
                (i as f64).log10().floor() as usize + 1
            };

            // Check if even number of digits
            if num_digits % 2 == 0 {
                // Calculate the divisor to split the number in half
                let divisor = 10_usize.pow((num_digits / 2) as u32);

                let first_half = i / divisor;
                let second_half = i % divisor;

                if first_half == second_half {
                    invalid_ids.push(i);
                }
            }
        }

        invalid_ids
    }
}

fn main() -> Result<()> {
    let ranges = Ranges::try_from(aoc_util::init()?)?;
    let sum = ranges.0.iter().flat_map(Range::invalid_ids).sum::<usize>();

    println!("{sum}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<()> {
        let expected = vec![2, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0];
        let ranges = Ranges::try_from(aoc_util::init_test()?)?;

        for (expected, range) in expected.into_iter().zip(ranges.0) {
            assert_eq!(expected, range.invalid_ids().len());
        }

        Ok(())
    }
}
