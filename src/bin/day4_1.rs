use lib::day4::passport::Passport;
use lib::day4::runner;
use std::error::Error;

/* See https://adventofcode.com/2020/day/4 for details. */
fn main() -> Result<(), Box<dyn Error>> {
  // All we care about are required fields being present, which is checked at parse time.
  // If the type system says it's valid, it's valid.
  runner::run(|fields| Passport::from_fields(fields).is_ok())
}
