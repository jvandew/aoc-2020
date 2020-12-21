use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Error as IOError, Lines};

/* A simple function runner to handle the boilerplate of opening and reading an input
 * file.
 */
pub struct FileInputRunner {
  lines: Lines<BufReader<File>>,
}

impl FileInputRunner {
  pub fn new(filename: &str) -> Result<FileInputRunner, IOError> {
    let file = File::open(filename)?;
    let runner = FileInputRunner {
      lines: BufReader::new(file).lines(),
    };
    Ok(runner)
  }

  pub fn run<F: FnMut(Lines<BufReader<File>>) -> Result<(), Box<dyn Error>>>(
    self,
    mut f: F,
  ) -> Result<(), Box<dyn Error>> {
    f(self.lines)
  }
}
