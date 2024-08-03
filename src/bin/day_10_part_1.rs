use std::{
    ops::{Add, BitOr},
    str::FromStr,
};
use strum::{EnumIter, EnumString, FromRepr, IntoEnumIterator};

#[derive(Debug, FromRepr, EnumIter, Clone, Copy)]
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
enum CellType {
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

impl CellType {
    /// All the directions (`Connection`s) reachable from this cell type,
    /// represented with bit flags as a `u8`.
    ///
    /// `Ground` is 0 because starting from a `Ground` cell we can't reach
    /// anything.
    ///
    /// `Start` is all four directions because we can go anywhere from the
    /// starting position.
    fn connections(self) -> u8 {
        match self {
            Self::NsPipe => Connection::North | Connection::South,
            Self::EwPipe => Connection::West | Connection::East,
            Self::NeBend => Connection::North | Connection::East,
            Self::NwBend => Connection::North | Connection::West,
            Self::SwBend => Connection::South | Connection::West,
            Self::SeBend => Connection::South | Connection::East,
            Self::Ground => 0,
            Self::Start => {
                // The grouping here is necessary to prevent the evaluation of either
                // a `u8 | Connection` or `Connection | u8` expression, neither of
                // which is current supported. We could implement `BitOr` for these
                // combinations of types, but that seems like overkill at the moment.
                (Connection::North | Connection::South) | (Connection::East | Connection::West)
            }
        }
    }

    fn connection_from(self, incoming: Connection) -> Result<Connection, IllegalConnectionError> {
        // This should never be called with `Start` since it won't
        // actually work in that case.
        assert_ne!(
            self,
            Self::Start,
            "`connection_from(CellType::Start) doesn't actually work",
        );
        // `self.connections()` is all the connections/directions reachable from this point. `Ground`
        // returns no connections, and `Start` returns all four.
        //
        // `incoming.reverse()` is the reverse of the incoming direction, e.g., if we're coming
        // here by traveling `East`, reversing that will give us `West`.
        //
        // The bitwise negation `!incoming.reverse()` gives us all the directions _except_ the
        // reverse of our incoming direction. So in our example, this would give us north, south,
        // and west.
        //
        // Bitwise & of these will give us anything that's in both. In most cases `self.connections()`
        // will return two directions, one of which is the one direction not in `!incoming.reverse()`,
        // so we just get the remaining option, which is the outgoing direction that doesn't take
        // us back to where we came from. If we're at `Ground` we'll get nothing back since `self.connections()`
        // will return the "empty set".
        Connection::from_repr(self.connections() & !(incoming.reverse() as u8))
            .ok_or(IllegalConnectionError)
    }
}

#[derive(Debug, Copy, Clone)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

impl Add<Connection> for Pos {
    type Output = Result<Self, Self>;

    fn add(self, rhs: Connection) -> Self::Output {
        let Self { row, col } = self;
        Ok(match rhs {
            Connection::North => Self {
                row: row.checked_sub(1).ok_or(self)?,
                col,
            },
            Connection::East => Self {
                row,
                col: col.checked_add(1).ok_or(self)?,
            },
            Connection::South => Self {
                row: row.checked_add(1).ok_or(self)?,
                col,
            },
            Connection::West => Self {
                row,
                col: col.checked_sub(1).ok_or(self)?,
            },
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct Cell {
    cell_type: CellType,
    pos: Pos,
}

impl Cell {
    pub const fn new(cell_type: CellType, pos: Pos) -> Self {
        Self { cell_type, pos }
    }

    pub const fn new_from_coords(cell_type: CellType, row: usize, col: usize) -> Self {
        Self::new(cell_type, Pos::new(row, col))
    }

    pub const fn empty(pos: Pos) -> Self {
        Self::new(CellType::Ground, pos)
    }
}

#[derive(Debug)]
struct PipeMap {
    entries: Vec<Vec<Cell>>,
    start: Pos,
}

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
                        let cell_type = CellType::from_repr(c).ok_or(PipeMapParseError)?;
                        Ok(Cell::new_from_coords(cell_type, row_number, col_number))
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;
        let start_row = start_row.ok_or(PipeMapParseError)?;
        let start_col = start_col.ok_or(PipeMapParseError)?;
        let start = Pos::new(start_row, start_col);
        Ok(Self { entries, start })
    }
}

impl PipeMap {
    fn start_cell(&self) -> Result<Cell, IllegalConnectionError> {
        self.get(self.start).map_err(|_| IllegalConnectionError)
    }

    fn starting_options(&self) -> Result<(Cell, Vec<Connection>), IllegalConnectionError> {
        let start = self.start_cell()?;
        let start_options = Connection::iter()
            .filter(|c| {
                self.advance(start, *c)
                    .and_then(|cell| cell.cell_type.connection_from(*c))
                    .is_ok()
            })
            .collect::<Vec<_>>();
        assert_eq!(
            start_options.len(),
            2,
            "We didn't have two options: {start_options:#?}"
        );
        Ok((start, start_options))
    }

    fn get(&self, pos: Pos) -> Result<Cell, Pos> {
        self.entries
            .get(pos.row)
            .and_then(|row| row.get(pos.col))
            .copied()
            .ok_or(pos)
    }

    fn move_to(&self, cell: Cell, direction: Connection) -> Result<Cell, Pos> {
        self.get((cell.pos + direction)?)
    }

    fn advance(
        &self,
        start: Cell,
        current_direction: Connection,
    ) -> Result<Cell, IllegalConnectionError> {
        // Converting from `Pos` to `IllegalConnectionError` makes the error type match
        // the type generated by `connection_from`. I feel like this is a place where
        // using either miette or anyhow would make all this a lot tidier.
        self.move_to(start, current_direction)
            .map_err(|_| IllegalConnectionError)
    }

    fn half_cycle_length(&self) -> Result<u64, IllegalConnectionError> {
        let (start, start_options) = self.starting_options()?;

        let mut current_direction = start_options[0];
        let mut current_cell = self.advance(start, current_direction)?;
        let mut num_steps = 1;

        while current_cell.cell_type != CellType::Start {
            current_direction = current_cell.cell_type.connection_from(current_direction)?;
            current_cell = self.advance(current_cell, current_direction)?;
            num_steps += 1;
        }

        Ok(num_steps / 2)
    }
}

fn main() -> Result<(), PipeMapParseError> {
    let input = include_str!("../inputs/day_10.txt");
    let pipe_map = PipeMap::from_str(input)?;
    // println!("{pipe_map:#?}");
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
        let result = pipe_map.half_cycle_length().unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn check_second_test_input() {
        let input = include_str!("../inputs/day_10_test_2.txt");
        let pipe_map = PipeMap::from_str(input).unwrap();
        let result = pipe_map.half_cycle_length().unwrap();
        assert_eq!(result, 8);
    }

    #[test]
    fn check_full_input() {
        let input = include_str!("../inputs/day_10.txt");
        let pipe_map = PipeMap::from_str(input).unwrap();
        let result = pipe_map.half_cycle_length().unwrap();
        assert_eq!(result, 6886);
    }
}
