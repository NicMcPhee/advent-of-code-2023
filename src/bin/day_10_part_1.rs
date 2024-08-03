use std::{num::ParseIntError, ops::BitOr, str::FromStr};

use itertools::Itertools;
use miette::Diagnostic;
use strum::{EnumIter, EnumString, FromRepr, IntoEnumIterator};

#[derive(FromRepr, EnumIter, Clone, Copy)]
#[repr(u8)]
enum Connection {
    North = 0b1000,
    East = 0b0100,
    South = 0b0010,
    West = 0b0001,
}

impl Connection {
    const fn reverse(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
}

impl BitOr for Connection {
    type Output = u8;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as u8 | rhs as u8
    }
}

/*
   | is a vertical pipe connecting north and south.
   - is a horizontal pipe connecting east and west.
   L is a 90-degree bend connecting north and east.
   J is a 90-degree bend connecting north and west.
   7 is a 90-degree bend connecting south and west.
   F is a 90-degree bend connecting south and east.
   . is ground; there is no pipe in this tile.
   S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
*/
#[derive(EnumString, FromRepr, Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
enum Cell {
    #[strum(serialize = "|")]
    NsPipe = b'|',
    #[strum(serialize = "-")]
    EwPipe = b'-',
    #[strum(serialize = "L")]
    NeBend = b'L',
    #[strum(serialize = "J")]
    NwBend = b'J',
    #[strum(serialize = "7")]
    SwBend = b'7',
    #[strum(serialize = "F")]
    SeBend = b'F',
    #[strum(serialize = ".")]
    Ground = b'.',
    #[strum(serialize = "S")]
    Start = b'S',
}

#[derive(Debug)]
struct IllegalConnectionError;

impl Cell {
    fn connections(self) -> u8 {
        match self {
            Self::NsPipe => Connection::North | Connection::South,
            Self::EwPipe => Connection::West | Connection::East,
            Self::NeBend => Connection::North | Connection::East,
            Self::NwBend => Connection::North | Connection::West,
            Self::SwBend => Connection::South | Connection::West,
            Self::SeBend => Connection::South | Connection::East,
            Self::Ground | Self::Start => 0,
        }
    }

    fn connection_from(self, incoming: Connection) -> Result<Connection, IllegalConnectionError> {
        Connection::from_repr(self.connections() ^ (incoming.reverse() as u8))
            .ok_or(IllegalConnectionError)
    }
}

#[derive(Debug)]
struct PipeMap {
    entries: Vec<Vec<Cell>>,
    start_row: usize,
    start_col: usize,
}

// #[derive(thiserror::Error, Debug, Diagnostic)]
// enum PipeMapParseError {
//     // #[error("Error parsing a pipe map")]
//     // #[diagnostic(transparent)]
//     // ValueHistory(#[from] ValueHistoryParseError),
// }

#[derive(Debug)]
struct PipeMapParseError;

impl FromStr for PipeMap {
    type Err = PipeMapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start_row: Option<usize> = None;
        let mut start_col: Option<usize> = None;
        let entries = s
            .lines()
            .enumerate()
            .map(|(row_number, line)| {
                line.bytes()
                    .enumerate()
                    .map(|(col_number, c)| {
                        if c == b'S' {
                            start_row = Some(row_number);
                            start_col = Some(col_number);
                        };
                        Cell::from_repr(c).ok_or(PipeMapParseError)
                    })
                    .collect::<Result<Vec<_>, PipeMapParseError>>()
            })
            .collect::<Result<Vec<_>, PipeMapParseError>>()?;
        let start_row = start_row.ok_or(PipeMapParseError)?;
        let start_col = start_col.ok_or(PipeMapParseError)?;
        Ok(Self {
            entries,
            start_row,
            start_col,
        })
    }
}

// TODO: Add a `Pos` struct, and put that in `Cell` so we know where we are.

impl PipeMap {
    fn move_to(&self, cell: Cell, direction: Connection) -> Cell {
        match direction {
            Connection::North => self.entries[],
            Connection::East => todo!(),
            Connection::South => todo!(),
            Connection::West => todo!(),
        }
        todo!()
    }

    fn half_cycle_length(&self) -> Result<u64, IllegalConnectionError> {
        let start = self.entries[self.start_row][self.start_col];
        let start_options = Connection::iter()
            .filter(|c| start.connection_from(*c).is_ok())
            .collect::<Vec<_>>();
        assert_eq!(start_options.len(), 2);

        let mut current_direction = start_options[0];
        let mut current_cell = self.move_to(start, current_direction);
        let mut num_steps = 1;

        while current_cell != Cell::Start {
            current_direction = current_cell.connection_from(current_direction)?;
            current_cell = self.move_to(current_cell, current_direction);
            num_steps += 1;
        }

        Ok(num_steps / 2)
    }
}

fn main() -> Result<(), PipeMapParseError> {
    let input = include_str!("../inputs/day_10_test_1.txt");
    let pipe_map = PipeMap::from_str(input)?;
    println!("{pipe_map:#?}");
    let result = pipe_map.half_cycle_length();
    println!("Result: {}", result.unwrap());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_first_test_input() {
        let input = include_str!("../inputs/day_10_test_1.txt");
        let pipe_map = PipeMap::from_str(input).unwrap();
        let result = pipe_map.half_cycle_length();
        assert_eq!(result, 4);
    }

    #[test]
    fn check_second_test_input() {
        let input = include_str!("../inputs/day_10_test_2.txt");
        let pipe_map = PipeMap::from_str(input).unwrap();
        let result = pipe_map.half_cycle_length();
        assert_eq!(result, 8);
    }

    #[test]
    fn check_full_input() {
        let input = include_str!("../inputs/day_10.txt");
        let pipe_map = PipeMap::from_str(input).unwrap();
        let result = pipe_map.half_cycle_length();
        assert_eq!(result, 923);
    }
}
