use lib::day2::policy_password::PolicyAndPassword;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/* See https://adventofcode.com/2020/day/2 for details. */
fn main() -> Result<(), Box<dyn Error>> {
  let filename = "resources/day2_1/input.txt";

  let file = File::open(filename)?;
  let lines = BufReader::new(file).lines();

  let mut valid_count = 0;
  for line in lines {
    let policy_password = PolicyAndPassword::parse(line?)?;
    let occurances = policy_password
      .password
      .chars()
      .filter(|character| character == &policy_password.character)
      .count();

    let valid =
      policy_password.min_times <= occurances && occurances <= policy_password.max_times;
    if valid {
      valid_count += 1;
    }
  }

  println!("The answer is: {:?}", valid_count);
  Ok(())
}
