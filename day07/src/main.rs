use anyhow::Result;
use log::trace;
use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone)]
enum Manifold {
    Start,
    Splitter,
    Beam,
    Empty,
}

impl From<char> for Manifold {
    fn from(value: char) -> Self {
        match value {
            'S' => Self::Start,
            '^' => Self::Splitter,
            '|' => Self::Beam,
            _ => Self::Empty,
        }
    }
}

impl Debug for Manifold {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Manifold {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Start => 'S',
                Self::Splitter => '^',
                Self::Beam => '|',
                Self::Empty => '.',
            }
        )
    }
}

#[derive(Debug, Default)]
struct Manifolds {
    manifolds: Vec<Vec<Manifold>>,
    index: usize,
    splits: usize,
}

impl Manifolds {
    fn advance(mut self) -> Result<Self, Self> {
        let index = self.index;

        if index > self.manifolds.len() - 1 {
            return Err(self);
        }

        let len = self.manifolds[index].len();
        for i in 0..len {
            match self.manifolds[index][i] {
                Manifold::Splitter => {
                    if index > 0 && matches!(self.manifolds[index - 1][i], Manifold::Beam) {
                        if i > 0 {
                            self.manifolds[index][i - 1] = Manifold::Beam;
                        }

                        if i + 1 < len {
                            self.manifolds[index][i + 1] = Manifold::Beam;
                        }

                        trace!("Splitting at {i}, {index}");
                        self.splits += 1;
                    }
                }
                Manifold::Empty => {
                    if index > 0
                        && matches!(
                            self.manifolds[index - 1][i],
                            Manifold::Start | Manifold::Beam
                        )
                    {
                        self.manifolds[index][i] = Manifold::Beam
                    }
                }
                Manifold::Start | Manifold::Beam => {}
            }
        }

        self.index += 1;

        Ok(self)
    }

    fn simulate(self) -> usize {
        let mut manifolds = self;
        loop {
            match manifolds.advance() {
                Ok(m) => manifolds = m,
                Err(m) => {
                    return m.splits;
                }
            }
        }
    }
}

impl TryFrom<Vec<String>> for Manifolds {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut manifolds = Manifolds::default();

        for line in value {
            let mut manifold_line = Vec::new();

            for c in line.chars() {
                manifold_line.push(Manifold::from(c));
            }

            manifolds.manifolds.push(manifold_line);
        }

        Ok(manifolds)
    }
}

fn main() -> Result<()> {
    let manifolds = Manifolds::try_from(aoc_util::init()?)?;

    let splits = manifolds.simulate();
    println!("{splits}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<()> {
        let manifolds = Manifolds::try_from(aoc_util::init_test()?)?;

        assert_eq!(21, manifolds.simulate());

        Ok(())
    }
}
