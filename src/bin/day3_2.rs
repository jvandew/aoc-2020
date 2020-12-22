use lib::day3::grid::{Grid, Slope};
use lib::runner::file_input_runner::FileInputRunner;
use std::error::Error;

/* See https://adventofcode.com/2020/day/3#part2 for details. */
fn main() -> Result<(), Box<dyn Error>> {
  let runner = FileInputRunner::new("resources/day3/input.txt")?;
  runner.run(|lines| {
    let grid = Grid::parse(lines)?;

    let slopes = vec![
      Slope { right: 1, down: 1 },
      Slope { right: 3, down: 1 },
      Slope { right: 5, down: 1 },
      Slope { right: 7, down: 1 },
      Slope { right: 1, down: 2 },
    ];

    let answer = slopes
      .iter()
      .map(|slope| {
        let tree_count = grid.traverse_and_count_trees(slope);
        println!("{:?} tree count: {:?}", slope, tree_count);
        tree_count
      })
      .product::<u32>();
    println!("The answer is: {:?}", answer);
    Ok(())
  })
}
