use lib::day5::seat::Seat;
use lib::runner::file_input_runner::FileInputRunner;
use std::error::Error;

/* See https://adventofcode.com/2020/day/5 for details. */
fn main() -> Result<(), Box<dyn Error>> {
  let runner = FileInputRunner::new("resources/day5/input.txt")?;
  runner.run(|lines| {
    let max_id_opt = lines
      .map(|line| -> Result<u16, Box<dyn Error>> {
        Ok(Seat::parse(line?.as_str())?.id())
      })
      .flat_map(|id_result| id_result.ok())
      .max();

    max_id_opt
      .ok_or_else(|| "Answer not found".into())
      .map(|max_id| println!("The answer is: {:?}", max_id))
  })
}
