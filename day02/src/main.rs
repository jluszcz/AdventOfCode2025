use anyhow::{bail, Result};
use log::debug;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug)]
struct Ranges(Vec<Range>);

impl Ranges {
    fn invalid_id_sum(&self) -> usize {
        self.0.iter().flat_map(Range::invalid_ids).sum()
    }
}

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
            if Self::is_invalid_id(i) {
                invalid_ids.push(i);
            }
        }

        debug!("{:?}", invalid_ids);
        invalid_ids
    }

    fn is_invalid_id(id: usize) -> bool {
        let s = id.to_string();
        let len = s.len();

        // Try all possible pattern lengths from 1 to len/2
        for pattern_len in 1..=len / 2 {
            if len.is_multiple_of(pattern_len) {
                let pattern = &s[0..pattern_len];
                let repeats = len / pattern_len;

                // Pattern must repeat at least twice
                if repeats >= 2 && pattern.repeat(repeats) == s {
                    return true;
                }
            }
        }

        false
    }
}

fn main() -> Result<()> {
    let ranges = Ranges::try_from(aoc_util::init()?)?;
    let sum = ranges.invalid_id_sum();

    println!("{sum}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_invalid() -> Result<()> {
        assert!(Range::is_invalid_id(12341234));
        assert!(Range::is_invalid_id(123123123));
        assert!(Range::is_invalid_id(1212121212));
        assert!(Range::is_invalid_id(1111111));

        Ok(())
    }

    #[test]
    fn example() -> Result<()> {
        let expected = vec![2, 2, 2, 1, 1, 0, 1, 1, 1, 1, 1];
        let ranges = Ranges::try_from(aoc_util::init_test()?)?;

        for (expected, range) in expected.into_iter().zip(&ranges.0) {
            assert_eq!(expected, range.invalid_ids().len());
        }

        assert_eq!(4174379265, ranges.invalid_id_sum());

        Ok(())
    }
}
