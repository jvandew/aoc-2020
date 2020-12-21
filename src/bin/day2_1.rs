use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

lazy_static! {
  static ref POLICY_PASSWORD_REGEX: Regex = Regex::new(
    r"^(?P<min_times>\d+)-(?P<max_times>\d+) (?P<character>[a-z]): (?P<password>[a-z]+)$",
  )
  .unwrap();
}

#[derive(Debug)]
struct PolicyAndPassword {
  min_times: usize,
  max_times: usize,
  character: char,
  password: String,
}

impl PolicyAndPassword {
  fn get_capture_group<'line>(
    captures: &Captures<'line>,
    name: &str,
    line_string: &str,
  ) -> Result<&'line str, String> {
    captures
      .name(name)
      .ok_or_else(|| {
        format!(
          "'{:?}' capture group not found in policy/password: {:?}",
          name, line_string,
        )
      })
      .map(|capture| capture.as_str())
  }

  fn parse<'a>(line_string: String) -> Result<PolicyAndPassword, Box<dyn Error>> {
    let captures = POLICY_PASSWORD_REGEX
      .captures(&line_string)
      .ok_or_else(|| format!("Invalid policy/password: '{:?}'", line_string))?;

    let min_times = {
      PolicyAndPassword::get_capture_group(&captures, "min_times", &line_string)?
        .parse::<usize>()?
    };
    let max_times = {
      PolicyAndPassword::get_capture_group(&captures, "max_times", &line_string)?
        .parse::<usize>()?
    };
    let character = {
      PolicyAndPassword::get_capture_group(&captures, "character", &line_string)?
        .chars()
        .next()
        .ok_or_else(|| {
          format!(
            "No policy character found in policy/password: {:?}",
            line_string,
          )
        })?
    };
    let password = {
      PolicyAndPassword::get_capture_group(&captures, "password", &line_string)?
        .to_string()
    };

    let policy_password = PolicyAndPassword {
      min_times,
      max_times,
      character,
      password,
    };
    Ok(policy_password)
  }
}

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
