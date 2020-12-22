use std::io::{BufRead, Error as IOError, ErrorKind as IOErrorKind, Lines};

const GRID_WIDTH: usize = 31;
const GRID_HEIGHT: usize = 323;
const TREE_CHAR: char = '#';
const EMPTY_CHAR: char = '.';

#[derive(Debug)]
pub struct Slope {
  pub right: usize,
  pub down: usize,
}

#[derive(Debug)]
enum GridSquare {
  Empty,
  Tree,
}

impl GridSquare {
  fn from_char(character: char) -> Result<GridSquare, IOError> {
    match character {
      EMPTY_CHAR => Ok(GridSquare::Empty),
      TREE_CHAR => Ok(GridSquare::Tree),
      other => Err(IOError::new(
        IOErrorKind::InvalidData,
        format!("Invalid grid character: '{:?}'", other),
      )),
    }
  }
}

#[derive(Debug)]
struct GridRow {
  squares: Vec<GridSquare>,
}

impl GridRow {
  fn parse(line: String) -> Result<GridRow, IOError> {
    let squares = line
      .chars()
      .flat_map(|character| GridSquare::from_char(character))
      .collect();
    Ok(GridRow { squares })
  }
}

#[derive(Debug)]
pub struct Grid {
  rows: Vec<GridRow>,
}

impl Grid {
  pub fn parse<L: BufRead>(lines: Lines<L>) -> Result<Grid, IOError> {
    let rows = lines.flat_map(|line| GridRow::parse(line?)).collect();
    Ok(Grid { rows })
  }

  pub fn traverse_and_count_trees(&self, slope: &Slope) -> u32 {
    let mut row = 0;
    let mut column = 0;
    let mut tree_count = 0;

    while row < GRID_HEIGHT - slope.down {
      column = (column + slope.right) % GRID_WIDTH;
      row += slope.down;

      match self.rows[row].squares[column] {
        GridSquare::Empty => (),
        GridSquare::Tree => tree_count += 1,
      };
    }

    tree_count
  }
}
