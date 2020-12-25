use lib::runner::file_input_runner::FileInputRunner;
use std::collections::HashSet;
use std::error::Error;

/* See https://adventofcode.com/2020/day/1#part2 for details. */
fn main() -> Result<(), Box<dyn Error>> {
  let runner = FileInputRunner::new("resources/day1/input.txt")?;
  runner.run(|lines| {
    let expenses: HashSet<i32> = lines
      .flat_map(|line| line)
      .flat_map(|line_string| line_string.parse::<i32>())
      .collect();

    let mut answer_opt = None;
    'loops: for expense1 in &expenses {
      for expense2 in &expenses {
        let inverse_expense = 2020 - expense1 - expense2;
        if expenses.contains(&inverse_expense) {
          answer_opt = Some(inverse_expense * expense1 * expense2);
          break 'loops;
        }
      }
    }

    answer_opt
      .ok_or_else(|| "Answer not found".into())
      .map(|answer| println!("The answer is: {:?}", answer))
  })
}
