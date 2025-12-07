use anyhow::Result;
use log::{debug, trace};
use std::collections::HashSet;
use std::fmt::Debug;

#[derive(Debug)]
struct Grid(Vec<Vec<bool>>);

impl TryFrom<Vec<String>> for Grid {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut grid = Vec::new();

        for (y, row) in value.iter().enumerate() {
            grid.push(vec![false; row.len()]);

            for (x, col) in row.chars().enumerate() {
                grid[y][x] = col == '@';
            }
        }

        Ok(Self(grid))
    }
}

impl Grid {
    fn is_roll_accessible(&self, x: usize, y: usize) -> bool {
        if !self.0[y][x] {
            trace!("No roll at {x}, {y}");
            return false;
        }

        let adj_count = aoc_util::grid::neighbors(&self.0, x, y, true)
            .into_iter()
            .filter(|n| {
                let neighbor_is_roll = self.0[n.position.1][n.position.0];
                trace!("Neighbor at {x}, {y} is roll: {neighbor_is_roll}");
                neighbor_is_roll
            })
            .count();

        debug!("{x}, {y} has {adj_count} adjacent roll(s)");
        adj_count < 4
    }

    fn remove_accessible_rolls(&mut self) -> usize {
        let mut count = 0;

        let mut to_remove = HashSet::new();

        for x in 0..self.0.len() {
            for y in 0..self.0[x].len() {
                if self.is_roll_accessible(x, y) {
                    to_remove.insert((x, y));
                    count += 1;
                }
            }
        }

        to_remove.iter().for_each(|&(x, y)| self.0[y][x] = false);

        count
    }

    fn accessible_roll_count(&mut self) -> usize {
        let mut count = 0;

        loop {
            let accessible_rolls = self.remove_accessible_rolls();
            count += accessible_rolls;

            if accessible_rolls == 0 {
                break;
            }
        }

        count
    }
}

fn main() -> Result<()> {
    let mut grid = Grid::try_from(aoc_util::init()?)?;

    let accessible_roll_count = grid.accessible_roll_count();
    println!("{accessible_roll_count}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<()> {
        let mut grid = Grid::try_from(aoc_util::init_test()?)?;

        assert_eq!(43, grid.accessible_roll_count());

        Ok(())
    }
}
