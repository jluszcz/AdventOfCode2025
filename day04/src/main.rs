use anyhow::Result;
use log::{debug, trace};
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

    fn accessible_roll_count(&self) -> usize {
        let mut count = 0;

        for x in 0..self.0.len() {
            for y in 0..self.0[x].len() {
                if self.is_roll_accessible(x, y) {
                    count += 1;
                }
            }
        }

        count
    }
}

fn main() -> Result<()> {
    let grid = Grid::try_from(aoc_util::init()?)?;

    let accessible_roll_count = grid.accessible_roll_count();
    println!("{accessible_roll_count}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<()> {
        let grid = Grid::try_from(aoc_util::init_test()?)?;

        assert_eq!(13, grid.accessible_roll_count());

        Ok(())
    }
}
