use lib::day4::passport::Passport;
use lib::day4::runner;
use regex::Regex;
use std::error::Error;

/* See https://adventofcode.com/2020/day/4#part2 for details. */
fn main() -> Result<(), Box<dyn Error>> {
  let year_regex = Regex::new(r"^(?P<year>\d{4})$")?;
  let height_regex = Regex::new(r"^(?P<height>\d+)(?P<unit>cm|in)$")?;
  let hair_color_regex = Regex::new(r"^#[a-f0-9]{6}$")?;
  let eye_color_regex = Regex::new(r"^(?:amb|blu|brn|gry|grn|hzl|oth)$")?;
  let passport_id_regex = Regex::new(r"^\d{9}$")?;

  let year_is_valid = |year_str: &str, range_start: u16, range_end: u16| -> bool {
    year_regex
      .captures(year_str)
      .and_then(|captures| captures.name("year"))
      .and_then(|year_match| {
        year_match
          .as_str()
          .parse::<u16>()
          .ok()
          .map(|birth_year| range_start <= birth_year && birth_year <= range_end)
      })
      .unwrap_or(false)
  };

  runner::run(|fields| match Passport::from_fields(fields) {
    Err(_) => false,
    Ok(passport) => {
      let birth_year_valid = year_is_valid(&passport.birth_year, 1920, 2002);
      let issue_year_valid = year_is_valid(&passport.issue_year, 2010, 2020);
      let expiration_year_valid = year_is_valid(&passport.expiration_year, 2020, 2030);

      let height_valid = height_regex
        .captures(&passport.height)
        .and_then(|captures| {
          captures.name("height").and_then(|height_match| {
            height_match.as_str().parse::<u8>().ok().map(|height| {
              match captures.name("unit").map(|unit_match| unit_match.as_str()) {
                Some("cm") => 150 <= height && height <= 193,
                Some("in") => 59 <= height && height <= 76,
                _ => false,
              }
            })
          })
        })
        .unwrap_or(false);

      let hair_color_valid = hair_color_regex.is_match(&passport.hair_color);
      let eye_color_valid = eye_color_regex.is_match(&passport.eye_color);
      let passport_id_valid = passport_id_regex.is_match(&passport.passport_id);

      birth_year_valid
        && issue_year_valid
        && expiration_year_valid
        && height_valid
        && hair_color_valid
        && eye_color_valid
        && passport_id_valid
    }
  })
}
