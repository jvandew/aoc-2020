use super::policy_password::PolicyAndPassword;
use crate::runner::file_input_runner::FileInputRunner;
use std::error::Error;

/* See https://adventofcode.com/2020/day/2 for details. */
pub fn run<F: Fn(PolicyAndPassword) -> bool>(is_valid: F) -> Result<(), Box<dyn Error>> {
  let runner = FileInputRunner::new("resources/day2/input.txt")?;
  runner.run(|lines| {
    let mut valid_count = 0;

    for line in lines {
      let policy_password = PolicyAndPassword::parse(line?)?;
      if is_valid(policy_password) {
        valid_count += 1;
      }
    }

    println!("The answer is: {:?}", valid_count);
    Ok(())
  })
}
