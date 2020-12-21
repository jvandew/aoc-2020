use lib::day2::runner;
use std::error::Error;

/* See https://adventofcode.com/2020/day/2#part2 for details. */
fn main() -> Result<(), Box<dyn Error>> {
  runner::run(|policy| {
    // NOTE(jacob): character iterator is shared here, so the second call must keep in
    //    mind how many characters were already consumed.
    let mut pw_chars = policy.password.chars();
    let char1 = pw_chars
      .nth(policy.min_times - 1)
      .ok_or_else(|| format!("Invalid password policy?: {:?}", policy))?;
    let char2 = pw_chars
      .nth(policy.max_times - policy.min_times - 1)
      .ok_or_else(|| format!("Invalid password policy?: {:?}", policy))?;

    Ok((char1 == policy.character || char2 == policy.character) && char1 != char2)
  })
}
