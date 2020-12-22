use lib::day3::grid::{Grid, Slope};
use lib::runner::file_input_runner::FileInputRunner;
use std::error::Error;

/* See https://adventofcode.com/2020/day/3 for details. */
fn main() -> Result<(), Box<dyn Error>> {
  let runner = FileInputRunner::new("resources/day3/input.txt")?;
  runner.run(|lines| {
    let grid = Grid::parse(lines)?;

    let slope = Slope { right: 3, down: 1 };

    let tree_count = grid.traverse_and_count_trees(&slope);
    println!("The answer is: {:?}", tree_count);
    Ok(())
  })
}
