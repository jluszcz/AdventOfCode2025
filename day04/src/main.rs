use anyhow::Result;
use aoc_util::grid::Grid;
use aoc_util::math::two_dimensional::Point;
use log::{debug, trace};
use std::fmt::Debug;

#[derive(Debug)]
struct Floor(Grid<bool>);

impl TryFrom<Vec<String>> for Floor {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut grid = Vec::new();

        for row in value.iter() {
            let mut grid_row = Vec::new();
            for col in row.chars() {
                grid_row.push(col == '@');
            }
            grid.push(grid_row);
        }

        Ok(Self(Grid::try_from(grid)?))
    }
}

impl Floor {
    fn is_roll_accessible(&self, position: Point) -> bool {
        if !self.0[position] {
            trace!("No roll at {position:?}");
            return false;
        }

        let adj_count = aoc_util::grid::neighbors(&self.0, position, true)
            .into_iter()
            .filter(|n| {
                let neighbor_is_roll = self.0[n.position];
                trace!("Neighbor at {position:?} is roll: {neighbor_is_roll}");
                neighbor_is_roll
            })
            .count();

        debug!("{position:?} has {adj_count} adjacent roll(s)");
        adj_count < 4
    }

    fn remove_accessible_rolls(&mut self) -> usize {
        let mut count = 0;

        let mut to_remove = Vec::new();

        for y in 0..self.0.height() {
            for x in 0..self.0.width() {
                let position = Point::new(x, y);
                if self.is_roll_accessible(position) {
                    to_remove.push(position);
                    count += 1;
                }
            }
        }

        to_remove
            .iter()
            .for_each(|&position| self.0[position] = false);

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
    let mut grid = Floor::try_from(aoc_util::init()?)?;

    let accessible_roll_count = grid.accessible_roll_count();
    println!("{accessible_roll_count}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<()> {
        let mut grid = Floor::try_from(aoc_util::init_test()?)?;

        assert_eq!(43, grid.accessible_roll_count());

        Ok(())
    }
}
