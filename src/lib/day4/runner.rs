use crate::runner::file_input_runner::FileInputRunner;
use std::collections::HashMap;
use std::error::Error;

/* See https://adventofcode.com/2020/day/2 for details. */
pub fn run<F: Fn(&HashMap<String, String>) -> bool>(
  is_valid: F,
) -> Result<(), Box<dyn Error>> {
  let runner = FileInputRunner::new("resources/day4/input.txt")?;
  runner.run(|lines| {
    let (_, valid_count) = lines.fold(
      Ok((HashMap::with_capacity(8), 0)),
      |prev_result, line| -> Result<(HashMap<String, String>, u32), Box<dyn Error>> {
        prev_result.and_then(|(mut passport_fields, valid_count)| {
          match line?.as_str() {
            "" => {
              let new_valid_count = {
                if is_valid(&passport_fields) {
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
                if let [name, value] = field.split(':').collect::<Vec<&str>>()[..] {
                  // NOTE: will overwrite duplicate field names!
                  passport_fields.insert(name.to_string(), value.to_string());
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
