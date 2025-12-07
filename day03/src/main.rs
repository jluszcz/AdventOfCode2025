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
        const RESULT_LENGTH: usize = 12;

        let digits: Vec<char> = self.0.chars().filter(|c| c.is_ascii_digit()).collect();
        let n = digits.len();

        if n < RESULT_LENGTH {
            return 0; // Not enough digits
        }

        let mut result = String::new();
        let mut current_pos = 0;

        for i in 0..RESULT_LENGTH {
            // How many more digits do we need after this one?
            let remaining = RESULT_LENGTH - i - 1;
            // We can look from current_pos to n - remaining
            let end_pos = n - remaining;

            // Find the maximum digit in this range
            let mut max_digit = digits[current_pos];
            let mut max_pos = current_pos;

            for (j, _) in digits.iter().enumerate().take(end_pos).skip(current_pos) {
                if digits[j] > max_digit {
                    max_digit = digits[j];
                    max_pos = j;
                }
            }

            result.push(max_digit);
            current_pos = max_pos + 1;
        }

        result.parse().unwrap_or(0)
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
        assert_eq!(
            987654321111,
            Bank::from_str("987654321111111")?.largest_joltage()
        );
        assert_eq!(
            811111111119,
            Bank::from_str("811111111111119")?.largest_joltage()
        );
        assert_eq!(
            434234234278,
            Bank::from_str("234234234234278")?.largest_joltage()
        );
        assert_eq!(
            888911112111,
            Bank::from_str("818181911112111")?.largest_joltage()
        );

        Ok(())
    }
}
