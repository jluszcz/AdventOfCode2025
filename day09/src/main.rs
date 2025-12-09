use anyhow::Result;
use aoc_util::grid::Point;

#[derive(Debug, Clone)]
struct Floor(Vec<Point>);

impl TryFrom<Vec<String>> for Floor {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let mut points = Vec::new();
        for line in value {
            points.push(line.parse::<Point>()?);
        }

        Ok(Floor(points))
    }
}

impl Floor {
    fn largest_rectangle(&self) -> usize {
        // Find the largest rectangle by checking all pairs of points
        let mut max_area = 0;
        for i in 0..self.0.len() {
            let p1 = self.0[i];

            for j in (i + 1)..self.0.len() {
                let p2 = self.0[j];

                let width = p1.x.abs_diff(p2.x) + 1;
                let height = p1.y.abs_diff(p2.y) + 1;

                let area = width * height;
                max_area = max_area.max(area);
            }
        }

        max_area
    }
}

fn main() -> Result<()> {
    let floor = Floor::try_from(aoc_util::init()?)?;

    let largest_rectangle = floor.largest_rectangle();
    println!("{largest_rectangle}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<()> {
        let floor = Floor::try_from(aoc_util::init_test()?)?;

        assert_eq!(50, floor.largest_rectangle());

        Ok(())
    }
}
