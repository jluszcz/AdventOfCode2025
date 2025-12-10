use anyhow::{anyhow, Result};
use std::str::FromStr;

#[derive(Debug, Default)]
struct Manuals(Vec<Manual>);

impl Manuals {
    fn min_presses(&self) -> usize {
        self.0.iter().map(|m| m.min_presses()).sum()
    }
}

impl TryFrom<Vec<String>> for Manuals {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let mut manuals = Manuals::default();

        for line in value {
            manuals.0.push(Manual::from_str(&line)?);
        }

        Ok(manuals)
    }
}

#[derive(Debug)]
struct Manual {
    goal: Vec<bool>,
    wiring_schematics: Vec<Vec<usize>>,
    _joltage_requirements: Vec<usize>,
}

impl Manual {
    fn min_presses(&self) -> usize {
        let num_lights = self.goal.len();
        let num_buttons = self.wiring_schematics.len();

        // Build augmented matrix [A | b] where:
        // - A[i][j] = 1 if button j toggles light i
        // - b[i] = goal state for light i
        let mut matrix = vec![vec![false; num_buttons + 1]; num_lights];

        for (button_idx, button) in self.wiring_schematics.iter().enumerate() {
            for &light_idx in button {
                matrix[light_idx][button_idx] = true;
            }
        }

        // Set goal column
        for (i, &goal_state) in self.goal.iter().enumerate() {
            matrix[i][num_buttons] = goal_state;
        }

        // Gaussian elimination over GF(2)
        let pivots = self.gauss_eliminate(&mut matrix, num_buttons);

        // Check if system is solvable
        for row in &matrix {
            // Check for contradiction: [0 0 0 ... 0 | 1]
            let all_zero = row[..num_buttons].iter().all(|&x| !x);
            if all_zero && row[num_buttons] {
                // No solution - this shouldn't happen for valid inputs
                return usize::MAX;
            }
        }

        // Find solution with minimum button presses
        self.find_min_solution(&matrix, &pivots, num_buttons)
    }

    fn gauss_eliminate(&self, matrix: &mut [Vec<bool>], num_cols: usize) -> Vec<Option<usize>> {
        let num_rows = matrix.len();
        let mut pivots = vec![None; num_rows];
        let mut current_row = 0;

        for col in 0..num_cols {
            // Find pivot
            let pivot_row = (current_row..num_rows).find(|&row| matrix[row][col]);

            if let Some(pivot_row) = pivot_row {
                // Swap rows if needed
                if pivot_row != current_row {
                    matrix.swap(pivot_row, current_row);
                }

                pivots[current_row] = Some(col);

                // Eliminate all other rows (including above, for RREF)
                for row in 0..num_rows {
                    if row != current_row && matrix[row][col] {
                        // XOR this row with current_row
                        // Use split_at_mut to avoid multiple mutable borrows
                        if row < current_row {
                            let (top, bottom) = matrix.split_at_mut(current_row);
                            for (a, &b) in top[row].iter_mut().zip(&bottom[0]).take(num_cols + 1) {
                                *a ^= b;
                            }
                        } else {
                            let (top, bottom) = matrix.split_at_mut(row);
                            for (a, &b) in bottom[0].iter_mut().zip(&top[current_row]).take(num_cols + 1) {
                                *a ^= b;
                            }
                        }
                    }
                }

                current_row += 1;
            }
        }

        pivots
    }

    fn find_min_solution(
        &self,
        matrix: &[Vec<bool>],
        pivots: &[Option<usize>],
        num_buttons: usize,
    ) -> usize {
        // Identify free variables (columns without pivots)
        let pivot_cols: Vec<usize> = pivots.iter().filter_map(|&p| p).collect();
        let mut free_vars = vec![];
        for col in 0..num_buttons {
            if !pivot_cols.contains(&col) {
                free_vars.push(col);
            }
        }

        // If no free variables, we have a unique solution
        if free_vars.is_empty() {
            let mut solution = vec![false; num_buttons];
            for (row_idx, &pivot) in pivots.iter().enumerate() {
                if let Some(col) = pivot {
                    solution[col] = matrix[row_idx][num_buttons];
                }
            }
            return solution.iter().filter(|&&x| x).count();
        }

        // Multiple solutions exist - try all combinations of free variables
        let num_free = free_vars.len();
        let mut min_presses = usize::MAX;

        for mask in 0..(1 << num_free) {
            let mut solution = vec![false; num_buttons];

            // Set free variables according to mask
            for (i, &free_var) in free_vars.iter().enumerate() {
                solution[free_var] = (mask >> i) & 1 == 1;
            }

            // Calculate dependent variables (pivot columns)
            for (row_idx, &pivot) in pivots.iter().enumerate() {
                if let Some(col) = pivot {
                    let mut val = matrix[row_idx][num_buttons];
                    // XOR with contributions from free variables
                    for (free_col, &is_set) in solution.iter().enumerate() {
                        if is_set && matrix[row_idx][free_col] {
                            val ^= true;
                        }
                    }
                    solution[col] = val;
                }
            }

            let presses = solution.iter().filter(|&&x| x).count();
            min_presses = min_presses.min(presses);
        }

        min_presses
    }
}

impl FromStr for Manual {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        // Parse goal from [.##.]
        let goal_start = s.find('[').ok_or_else(|| anyhow!("No goal found"))?;
        let goal_end = s
            .find(']')
            .ok_or_else(|| anyhow!("No closing bracket for goal"))?;
        let goal_str = &s[goal_start + 1..goal_end];
        let goal = goal_str.chars().map(|c| c == '#').collect();

        // Parse wiring schematics from (1,3) (2) etc.
        let mut wiring_schematics = Vec::new();
        let after_goal = &s[goal_end + 1..];
        let before_braces = if let Some(brace_pos) = after_goal.find('{') {
            &after_goal[..brace_pos]
        } else {
            after_goal
        };

        for part in before_braces.split(')') {
            if let Some(paren_start) = part.find('(') {
                let nums_str = &part[paren_start + 1..];
                let nums: Vec<usize> = nums_str
                    .split(',')
                    .filter(|s| !s.trim().is_empty())
                    .map(|n| n.trim().parse())
                    .collect::<Result<Vec<_>, _>>()?;
                wiring_schematics.push(nums);
            }
        }

        // Parse joltage requirements from {3,5,4,7}
        let brace_start = s
            .find('{')
            .ok_or_else(|| anyhow!("No joltage requirements found"))?;
        let brace_end = s
            .find('}')
            .ok_or_else(|| anyhow!("No closing brace for joltage requirements"))?;
        let joltage_str = &s[brace_start + 1..brace_end];
        let joltage_requirements = joltage_str
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .map(|n| n.trim().parse())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Manual {
            goal,
            wiring_schematics,
            _joltage_requirements: joltage_requirements,
        })
    }
}

fn main() -> Result<()> {
    let manuals = Manuals::try_from(aoc_util::init()?)?;

    let min_presses = manuals.min_presses();
    println!("{min_presses}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_manual() -> Result<()> {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let manual: Manual = input.parse()?;

        assert_eq!(manual.goal, vec![false, true, true, false]);
        assert_eq!(manual.wiring_schematics.len(), 6);
        assert_eq!(manual.wiring_schematics[0], vec![3]);
        assert_eq!(manual.wiring_schematics[1], vec![1, 3]);
        assert_eq!(manual.wiring_schematics[2], vec![2]);
        assert_eq!(manual.wiring_schematics[3], vec![2, 3]);
        assert_eq!(manual.wiring_schematics[4], vec![0, 2]);
        assert_eq!(manual.wiring_schematics[5], vec![0, 1]);
        assert_eq!(manual._joltage_requirements, vec![3, 5, 4, 7]);

        Ok(())
    }

    #[test]
    fn min_presses() -> Result<()> {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let manual: Manual = input.parse()?;

        assert_eq!(2, manual.min_presses());

        Ok(())
    }

    #[test]
    fn example() -> Result<()> {
        let manuals = Manuals::try_from(aoc_util::init_test()?)?;

        assert_eq!(7, manuals.min_presses());

        Ok(())
    }
}
