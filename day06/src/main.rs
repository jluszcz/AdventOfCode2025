use anyhow::{bail, Result};
use log::trace;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn apply(&self, operands: &[usize]) -> usize {
        trace!("{self:?} -> {operands:?}");

        match self {
            Operator::Add => operands.iter().sum::<usize>(),
            Operator::Multiply => operands.iter().product::<usize>(),
        }
    }
}

impl Debug for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operator::Add => "+",
                Operator::Multiply => "*",
            }
        )
    }
}

impl FromStr for Operator {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Self::Add,
            "*" => Self::Multiply,
            _ => bail!("unknown operator: {s}"),
        })
    }
}

#[derive(Debug, Default)]
struct Worksheet {
    operands: Vec<Vec<usize>>,
    operators: Vec<Operator>,
}

impl Worksheet {
    fn apply_at(&self, index: usize) -> usize {
        let column: Vec<usize> = self.operands.iter().map(|row| row[index]).collect();
        self.operators[index].apply(&column)
    }

    fn grand_total(&self) -> usize {
        (0..self.operators.len()).map(|i| self.apply_at(i)).sum()
    }
}

impl TryFrom<Vec<String>> for Worksheet {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut worksheet = Worksheet::default();

        let mut count = None;

        for line in value.iter().take(value.len() - 1) {
            let mut operands = Vec::new();
            for v in line.split_ascii_whitespace() {
                operands.push(v.parse::<usize>()?);
            }

            if let Some(count) = count {
                if count != operands.len() {
                    bail!("wrong number of operands: {} != {}", count, operands.len());
                }
            } else {
                count = Some(operands.len());
            }

            worksheet.operands.push(operands);
        }

        for line in value.iter().skip(value.len() - 1) {
            let mut operators = Vec::new();
            for o in line.split_ascii_whitespace() {
                operators.push(Operator::from_str(o)?);
            }

            if let Some(count) = count
                && count != operators.len()
            {
                bail!(
                    "wrong number of operators: {} != {}",
                    count,
                    operators.len()
                );
            }

            worksheet.operators = operators;
        }

        Ok(worksheet)
    }
}

fn main() -> Result<()> {
    let worksheet = Worksheet::try_from(aoc_util::init()?)?;

    let grand_total = worksheet.grand_total();
    println!("{grand_total}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<()> {
        let worksheet = Worksheet::try_from(aoc_util::init_test()?)?;

        let expected = vec![33210, 490, 4243455, 401];

        for (i, expected) in expected.into_iter().enumerate() {
            assert_eq!(expected, worksheet.apply_at(i));
        }

        Ok(())
    }
}
