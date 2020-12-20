use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/* See https://adventofcode.com/2020/day/1#part2 for details. */
fn main() -> Result<(), Box<dyn Error>> {
  let filename = "resources/day1_1/input.txt";

  let file = File::open(filename)?;
  let expenses: HashSet<i32> = BufReader::new(file)
    .lines()
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
    .ok_or("Answer not found".into())
    .map(|answer| println!("The answer is: {:?}", answer))
}
