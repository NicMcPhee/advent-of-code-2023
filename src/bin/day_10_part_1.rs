use std::{num::ParseIntError, str::FromStr};

use itertools::Itertools;
use miette::Diagnostic;

// TODO: Either impl bitwise OR for `Connections` or use `BitBags` which will do that for us.

#[repr(u8)]
enum Connections {
    Nowhere = 0b0000,
    North = 0b1000,
    East = 0b0100,
    South = 0b0010,
    West = 0b0001,
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
#[repr(u8)]
enum Cell {
    NsPipe = Connections::North | Connections::South,
    EwPipe = Connections::West | Connections::East,
    NeBend = Connections::North | Connections::East,
    NwBend = Connections::North | Connections::West,
    SwBend = Connections::South | Connections::West,
    SeBend = Connections::South | Connections::East,
    Ground = Connections::Nowhere,
    Start = Connections::North | Connections::South | Connections::East | Connections::West,
}

struct PipeMap {
    entries: Vec<Vec<Cell>>,
}

#[derive(thiserror::Error, Debug, Diagnostic)]
enum PipeMapParseError {
    // #[error("Error parsing a pipe map")]
    // #[diagnostic(transparent)]
    // ValueHistory(#[from] ValueHistoryParseError),
}

impl FromStr for PipeMap {
    type Err = PipeMapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl PipeMap {
    fn half_cycle_length(&self) -> i64 {
        todo!()
    }
}

fn main() -> miette::Result<()> {
    let input = include_str!("../inputs/day_10.txt");
    let pipe_map = PipeMap::from_str(input)?;
    let result = pipe_map.half_cycle_length();
    println!("Result: {result}");

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
