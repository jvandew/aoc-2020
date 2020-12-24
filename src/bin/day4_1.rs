use lib::runner::file_input_runner::FileInputRunner;
use std::collections::HashSet;
use std::error::Error;
use std::io::Error as IOError;

/* See https://adventofcode.com/2020/day/4 for details. */
fn main() -> Result<(), Box<dyn Error>> {
  let runner = FileInputRunner::new("resources/day4/input.txt")?;

  let valid_fields = vec![
    "byr", // Birth Year
    "iyr", // Issue Year
    "eyr", // Expiration Year
    "hgt", // Height
    "hcl", // Hair Color
    "ecl", // Eye Color
    "pid", // Passport ID
    "cid", // Country ID
  ]
  .into_iter()
  .map(|str| str.to_string())
  .collect::<HashSet<String>>();

  runner.run(|lines| {
    let (_, valid_count) = lines.fold(
      Ok((HashSet::with_capacity(8), 0)),
      |prev_result, line| -> Result<(HashSet<String>, u32), IOError> {
        prev_result.and_then(|(mut passport_fields, valid_count)| {
          match line?.as_str() {
            "" => {
              let new_valid_count = {
                let present_fields = passport_fields
                  .intersection(&valid_fields)
                  .map(|string| string.as_str())
                  .collect::<HashSet<&str>>();

                let valid = (present_fields.len() == 7
                  && !present_fields.contains("cid"))
                  || present_fields.len() == 8;

                if valid {
                  valid_count + 1
                } else {
                  valid_count
                }
              };

              passport_fields.clear();
              Ok((passport_fields, new_valid_count))
            }

            fields => {
              for field in fields.split(' ') {
                if let Some(name) = field.split(':').next() {
                  // NOTE: will overwrite duplicate field names!
                  passport_fields.insert(name.to_string());
                }
              }
              Ok((passport_fields, valid_count))
            }
          }
        })
      },
    )?;

    println!("The answer is: {:?}", valid_count);
    Ok(())
  })
}
