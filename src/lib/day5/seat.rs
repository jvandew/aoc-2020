use std::num::ParseIntError;

#[derive(Debug)]
pub struct Seat {
  row: u8,
  column: u8,
}

impl Seat {
  pub fn parse(seating_code: &str) -> Result<Seat, ParseIntError> {
    let row_code = seating_code[..7].replace('F', "0").replace('B', "1");
    let row = u8::from_str_radix(&row_code, 2)?;

    let column_code = seating_code[7..10].replace('L', "0").replace('R', "1");
    let column = u8::from_str_radix(&column_code, 2)?;

    let seat = Seat { row, column };
    Ok(seat)
  }

  pub fn id(&self) -> u16 {
    u16::from(self.row) * 8 + u16::from(self.column)
  }
}
