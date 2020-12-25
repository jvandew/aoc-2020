use itertools::sorted;
use lib::day5::seat::Seat;
use lib::runner::file_input_runner::FileInputRunner;
use std::error::Error;

/* See https://adventofcode.com/2020/day/5#part2 for details. */
fn main() -> Result<(), Box<dyn Error>> {
  let runner = FileInputRunner::new("resources/day5/input.txt")?;
  runner.run(|lines| {
    let ids = lines
      .map(|line| -> Result<u16, Box<dyn Error>> {
        Ok(Seat::parse(line?.as_str())?.id())
      })
      .flat_map(|id_result| id_result.ok());

    // doesn't short circuit, but doing this mutably with a loop is gross
    let (missing_id_opt, _) = sorted(ids).fold(
      (None, None),
      |(missing_id_opt, previous_id_opt), next_id| {
        match (missing_id_opt, previous_id_opt) {
          // already found our missing ID; continue until iterator is exhausted
          (Some(_), _) => (missing_id_opt, None),

          // first ID we've seen; record it for the next iteration
          (None, None) => (None, Some(next_id)),

          // check previously seen ID for a gap
          (None, Some(previous_id)) => {
            if next_id == previous_id + 2 {
              (Some(previous_id + 1), None)
            } else {
              (None, Some(next_id))
            }
          }
        }
      },
    );

    missing_id_opt
      .ok_or_else(|| "Answer not found".into())
      .map(|missing_id| println!("The answer is: {:?}", missing_id))
  })
}
