use lib::day2::runner;
use std::error::Error;

/* See https://adventofcode.com/2020/day/2 for details. */
fn main() -> Result<(), Box<dyn Error>> {
  runner::run(|policy_password| {
    let occurances = policy_password
      .password
      .chars()
      .filter(|character| character == &policy_password.character)
      .count();

    policy_password.min_times <= occurances && occurances <= policy_password.max_times
  })
}
