// use lazy_static::lazy_static;
// use std::collections::{HashMap, HashSet};
use std::collections::HashMap;

// lazy_static! {
//   static ref REQUIRED_FIELDS: HashSet<&str> = vec![
//     "byr", // Birth Year
//     "iyr", // Issue Year
//     "eyr", // Expiration Year
//     "hgt", // Height
//     "hcl", // Hair Color
//     "ecl", // Eye Color
//     "pid", // Passport ID
//   ]
//   .into_iter()
//   .collect::<HashSet<&str>>();

//   static ref OPTIONAL_FIELDS: HashSet<&str> = vec![
//     "cid", // Country ID
//   ]
//   .into_iter()
//   .collect::<HashSet<&str>>();
// }

#[derive(Debug)]
pub struct Passport {
  pub birth_year: String,
  pub issue_year: String,
  pub expiration_year: String,
  pub height: String,
  pub hair_color: String,
  pub eye_color: String,
  pub passport_id: String,
  pub country_id_opt: Option<String>,
}

impl Passport {
  pub fn from_fields(fields: &HashMap<String, String>) -> Result<Passport, String> {
    let birth_year = fields
      .get("byr")
      .ok_or_else(|| "Passport missing required field 'byr'")?;
    let issue_year = fields
      .get("iyr")
      .ok_or_else(|| "Passport missing required field 'iyr'")?;
    let expiration_year = fields
      .get("eyr")
      .ok_or_else(|| "Passport missing required field 'eyr'")?;
    let height = fields
      .get("hgt")
      .ok_or_else(|| "Passport missing required field 'hgt'")?;
    let hair_color = fields
      .get("hcl")
      .ok_or_else(|| "Passport missing required field 'hcl'")?;
    let eye_color = fields
      .get("ecl")
      .ok_or_else(|| "Passport missing required field 'ecl'")?;
    let passport_id = fields
      .get("pid")
      .ok_or_else(|| "Passport missing required field 'pid'")?;
    let country_id_opt = fields.get("cid");

    let passport = Passport {
      birth_year: birth_year.to_string(),
      issue_year: issue_year.to_string(),
      expiration_year: expiration_year.to_string(),
      height: height.to_string(),
      hair_color: hair_color.to_string(),
      eye_color: eye_color.to_string(),
      passport_id: passport_id.to_string(),
      country_id_opt: country_id_opt.map(|country_id| country_id.to_string()),
    };

    Ok(passport)
  }
}
