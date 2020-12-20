use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/* See https://adventofcode.com/2020/day/1 for details. */
fn main() -> Result<(), Box<dyn Error>> {
  let filename = "resources/day1_1/input.txt";

  let mut expenses = HashSet::new();

  let file = File::open(filename)?;
  let lines = BufReader::new(file).lines();

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
    .ok_or("Answer not found".into())
    .map(|answer| println!("The answer is: {:?}", answer))
}
