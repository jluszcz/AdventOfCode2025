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
    #[allow(dead_code)]
    fn is_spoiled(&self, ingredient: usize) -> bool {
        for range in self.fresh_ranges.iter() {
            if range.contains(&ingredient) {
                return true;
            }
        }

        false
    }

    #[allow(dead_code)]
    fn fresh_count(&self) -> usize {
        self.ingredients
            .iter()
            .filter(|i| self.is_spoiled(**i))
            .count()
    }

    fn merged_ranges(&self) -> Vec<RangeInclusive<usize>> {
        let mut fresh_ranges = self.fresh_ranges.clone();
        fresh_ranges.sort_by(|a, b| a.start().cmp(b.start()).then(a.end().cmp(b.end())));

        // Merge overlapping or adjacent ranges
        let mut merged_ranges: Vec<RangeInclusive<usize>> = Vec::new();
        for range in fresh_ranges {
            if let Some(last) = merged_ranges.last_mut() {
                // Check if current range overlaps or is adjacent to the last merged range
                if *range.start() <= *last.end() + 1 {
                    // Merge by extending the end if needed
                    let new_end = (*last.end()).max(*range.end());
                    *last = *last.start()..=new_end;
                } else {
                    // No overlap, add as new range
                    merged_ranges.push(range);
                }
            } else {
                // First range
                merged_ranges.push(range);
            }
        }

        merged_ranges
    }

    fn fresh_id_count(&self) -> usize {
        let mut fresh_id_count = 0;

        for range in self.merged_ranges() {
            fresh_id_count += range.end() - range.start() + 1;
        }

        fresh_id_count
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

    let fresh_count = db.fresh_id_count();
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

        assert_eq!(14, db.fresh_id_count());

        Ok(())
    }
}
