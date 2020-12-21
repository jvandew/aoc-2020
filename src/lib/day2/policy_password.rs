use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::error::Error;

lazy_static! {
  static ref POLICY_PASSWORD_REGEX: Regex = Regex::new(
    r"^(?P<min_times>\d+)-(?P<max_times>\d+) (?P<character>[a-z]): (?P<password>[a-z]+)$",
  )
  .unwrap();
}

#[derive(Debug)]
pub struct PolicyAndPassword {
  pub min_times: usize,
  pub max_times: usize,
  pub character: char,
  pub password: String,
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

  pub fn parse<'a>(line_string: String) -> Result<PolicyAndPassword, Box<dyn Error>> {
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
