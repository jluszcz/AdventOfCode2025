use anyhow::Result;
use std::collections::HashSet;
use std::fmt::Debug;
use std::ops::RangeInclusive;

#[derive(Debug, Default)]
struct Database {
    fresh_ranges: Vec<RangeInclusive<usize>>,
    ingredients: HashSet<usize>,
}

impl Database {
    fn is_spoiled(&self, ingredient: usize) -> bool {
        for range in self.fresh_ranges.iter() {
            if range.contains(&ingredient) {
                return true;
            }
        }

        false
    }

    fn fresh_count(&self) -> usize {
        self.ingredients
            .iter()
            .filter(|i| self.is_spoiled(**i))
            .count()
    }
}

impl TryFrom<Vec<String>> for Database {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut database = Database::default();

        let mut ranges = true;
        for line in value.iter() {
            if line.is_empty() {
                ranges = false;
                continue;
            }

            if ranges {
                let (start, end) = line
                    .split_once('-')
                    .ok_or_else(|| anyhow::anyhow!("Invalid range"))?;

                database
                    .fresh_ranges
                    .push(RangeInclusive::new(start.parse()?, end.parse()?));
            } else {
                database.ingredients.insert(line.parse::<usize>()?);
            }
        }

        Ok(database)
    }
}

fn main() -> Result<()> {
    let db = Database::try_from(aoc_util::init()?)?;

    let fresh_count = db.fresh_count();
    println!("{fresh_count}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<()> {
        let db = Database::try_from(aoc_util::init_test()?)?;

        assert!(!db.is_spoiled(1));
        assert!(db.is_spoiled(5));
        assert!(!db.is_spoiled(8));
        assert!(db.is_spoiled(11));
        assert!(db.is_spoiled(17));
        assert!(!db.is_spoiled(32));

        Ok(())
    }
}
