use lib::day2::runner;
use std::error::Error;

/* See https://adventofcode.com/2020/day/2 for details. */
fn main() -> Result<(), Box<dyn Error>> {
  runner::run(|policy| {
    let occurances = policy
      .password
      .chars()
      .filter(|character| character == &policy.character)
      .count();

    Ok(policy.min_times <= occurances && occurances <= policy.max_times)
  })
}
