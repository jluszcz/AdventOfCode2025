use anyhow::Result;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug)]
struct Bank(String);

impl FromStr for Bank {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl Bank {
    fn largest_joltage(&self) -> usize {
        let digits: Vec<usize> = self
            .0
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as usize)
            .collect();

        let mut max_joltage = 0;

        for i in 0..digits.len() {
            for j in (i + 1)..digits.len() {
                let joltage = digits[i] * 10 + digits[j];
                max_joltage = max_joltage.max(joltage);
            }
        }

        max_joltage
    }
}

fn main() -> Result<()> {
    let sum = aoc_util::init()?
        .iter()
        .filter_map(|s| Bank::from_str(s).ok())
        .map(|b| b.largest_joltage())
        .sum::<usize>();

    println!("{sum}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<()> {
        assert_eq!(98, Bank::from_str("987654321111111")?.largest_joltage());
        assert_eq!(89, Bank::from_str("811111111111119")?.largest_joltage());
        assert_eq!(78, Bank::from_str("234234234234278")?.largest_joltage());
        assert_eq!(92, Bank::from_str("818181911112111")?.largest_joltage());

        Ok(())
    }
}
