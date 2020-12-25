use lib::runner::file_input_runner::FileInputRunner;
use std::collections::HashSet;
use std::error::Error;

/* See https://adventofcode.com/2020/day/1 for details. */
fn main() -> Result<(), Box<dyn Error>> {
  let runner = FileInputRunner::new("resources/day1/input.txt")?;
  runner.run(|lines| {
    let mut expenses = HashSet::new();
    let mut answer_opt = None;

    for line in lines {
      let expense = line?.parse::<i32>()?;
      let inverse_expense = 2020 - expense;

      if expenses.contains(&inverse_expense) {
        answer_opt = Some(inverse_expense * expense);
        break;
      } else {
        expenses.insert(expense);
      }
    }

    answer_opt
      .ok_or_else(|| "Answer not found".into())
      .map(|answer| println!("The answer is: {:?}", answer))
  })
}
