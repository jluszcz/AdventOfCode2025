use anyhow::Result;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() -> Result<()> {
        /*
        The dial starts by pointing at 50.
        The dial is rotated L68 to point at 82.
        The dial is rotated L30 to point at 52.
        The dial is rotated R48 to point at 0.
        The dial is rotated L5 to point at 95.
        The dial is rotated R60 to point at 55.
        The dial is rotated L55 to point at 0.
        The dial is rotated L1 to point at 99.
        The dial is rotated L99 to point at 0.
        The dial is rotated R14 to point at 14.
        The dial is rotated L82 to point at 32.
         */
        let input = aoc_util::init_test()?;

        Ok(())
    }
}
