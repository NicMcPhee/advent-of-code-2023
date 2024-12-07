use miette::Diagnostic;
use ndarray::{Array, Array2, ShapeError};
use std::{
    fmt::{Display, Write},
    ops::{Add, Index, IndexMut},
    str::FromStr,
};

#[derive(Debug, Diagnostic, thiserror::Error)]
enum ParseError {
    #[error("Tried to parse a pattern with no lines")]
    EmptyPattern,

    #[error(transparent)]
    ArrayShape(#[from] ShapeError),

    #[error("Illegal location character {0}")]
    IllegalLocation(char),
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
enum Tile {
    Slash,
    Backslash,
    Dash,
    Pipe,
    Empty,
}

impl Tile {
    const fn perpendicular(self, direction: CardinalDirection) -> bool {
        matches!(
            (self, direction),
            (
                Self::Dash,
                CardinalDirection::North | CardinalDirection::South
            ) | (
                Self::Pipe,
                CardinalDirection::East | CardinalDirection::West
            )
        )
    }
}

impl TryFrom<char> for Tile {
    type Error = ParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            '.' => Self::Empty,
            '/' => Self::Slash,
            '\\' => Self::Backslash,
            '|' => Self::Pipe,
            '-' => Self::Dash,
            c => return Err(ParseError::IllegalLocation(c)),
        })
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Slash => f.write_char('/'),
            Self::Backslash => f.write_char('\\'),
            Self::Dash => f.write_char('-'),
            Self::Pipe => f.write_char('|'),
            Self::Empty => f.write_char('.'),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CardinalDirection {
    North,
    South,
    East,
    West,
}

impl CardinalDirection {
    const fn reverse(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    const fn rotate_slash(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::North,
            Self::South => Self::West,
            Self::West => Self::South,
        }
    }

    const fn rotate_backslash(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::South,
            Self::South => Self::East,
            Self::West => Self::North,
        }
    }

    const fn split(self) -> [Self; 2] {
        match self {
            Self::East | Self::West => [Self::North, Self::South],
            Self::North | Self::South => [Self::East, Self::West],
        }
    }
}

type Position = (usize, usize);

impl Add<CardinalDirection> for Position {
    type Output = Option<Self>;

    fn add(self, rhs: CardinalDirection) -> Self::Output {
        let (row, col) = self;
        Some(match rhs {
            CardinalDirection::North => (row.checked_sub(1)?, col),
            CardinalDirection::South => (row.checked_add(1)?, col),
            CardinalDirection::East => (row, col.checked_add(1)?),
            CardinalDirection::West => (row, col.checked_sub(1)?),
        })
    }
}

#[expect(
    clippy::struct_excessive_bools,
    reason = "This is not a state machine like Clippy thinks"
)]
#[derive(Debug, Default, Copy, Clone)]
struct EnteredFrom {
    north: bool,
    south: bool,
    east: bool,
    west: bool,
}

impl EnteredFrom {
    pub const fn any(self) -> bool {
        self.north || self.south || self.east || self.west
    }
}

impl Index<CardinalDirection> for EnteredFrom {
    type Output = bool;

    fn index(&self, direction: CardinalDirection) -> &Self::Output {
        match direction {
            CardinalDirection::North => &self.north,
            CardinalDirection::South => &self.south,
            CardinalDirection::East => &self.east,
            CardinalDirection::West => &self.west,
        }
    }
}

impl IndexMut<CardinalDirection> for EnteredFrom {
    fn index_mut(&mut self, direction: CardinalDirection) -> &mut Self::Output {
        match direction {
            CardinalDirection::North => &mut self.north,
            CardinalDirection::South => &mut self.south,
            CardinalDirection::East => &mut self.east,
            CardinalDirection::West => &mut self.west,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Location {
    tile: Tile,
    entered_from: EnteredFrom,
}

impl Location {
    pub fn new(tile: Tile) -> Self {
        Self {
            tile,
            entered_from: EnteredFrom::default(),
        }
    }

    pub const fn energized(self) -> bool {
        self.entered_from.any()
    }
}

impl TryFrom<char> for Location {
    type Error = ParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Tile::try_from(c).map(Self::new)
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.tile, f)
    }
}

#[derive(Debug, Clone)]
struct Grid {
    array: Array2<Location>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.array.rows() {
            for location in row {
                Display::fmt(location, f)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Grid {
    fn new(num_columns: usize, locations: Vec<Location>) -> Result<Self, ParseError> {
        debug_assert_eq!(locations.len() % num_columns, 0);
        let num_rows = locations.len() / num_columns;
        let array = Array::from_shape_vec((num_rows, num_columns), locations)?;
        Ok(Self { array })
    }

    fn num_energized(&self) -> usize {
        self.array.iter().filter(|l| l.energized()).count()
    }

    fn maximize_energized(&self) -> usize {
        // For each side:
        //    Loop over all the entry points.
        //    Clone the grid and call shine_beam
        //    Get the `num_energized()` from the result grid
        //    maximize over those
        let nrows = self.array.nrows();
        let ncols = self.array.ncols();
        let mut result = usize::MIN;
        for row in 0..nrows {
            result = result.max(self.beam_energized((row, 0), CardinalDirection::East));
            result = result.max(self.beam_energized((row, ncols - 1), CardinalDirection::West));
        }
        for col in 0..ncols {
            result = result.max(self.beam_energized((0, col), CardinalDirection::South));
            result = result.max(self.beam_energized((nrows - 1, col), CardinalDirection::North));
        }
        result
    }

    fn beam_energized(&self, position: Position, direction: CardinalDirection) -> usize {
        let mut grid_clone = self.clone();
        grid_clone.shine_beam(position, direction);
        grid_clone.num_energized()
    }

    fn shine_beam(&mut self, position: Position, direction: CardinalDirection) {
        let location = &mut self.array[position];
        if location.entered_from[direction.reverse()] {
            return;
        }
        location.entered_from[direction.reverse()] = true;
        match location.tile {
            // If the tile is a mirror (`Slash` or `Backslash`), then rotate the direction of the beam
            // and continue one step in the new direction.
            Tile::Slash => self.step_and_shine(position, direction.rotate_slash()),
            Tile::Backslash => self.step_and_shine(position, direction.rotate_backslash()),
            // If the tile is a splitter (`Dash` or `Pipe`) and we strike it perpendicularly, then the beam
            // splits into two beams, each going perpendicular to the original beam, so we have to call `shine_beam`
            // on each of the new beams.
            tile @ (Tile::Dash | Tile::Pipe) if tile.perpendicular(direction) => {
                direction
                    .split()
                    .into_iter()
                    .for_each(|new_direction| self.step_and_shine(position, new_direction));
            }
            // If the tile is `Empty`, or it's `Dash` or `Pipe` but the beam is _not_ traveling in the perpendicular direction,
            // then the beam just passes through this grid location continuing in the same direction.
            _ => self.step_and_shine(position, direction),
        };
    }

    fn step(&self, position: Position, direction: CardinalDirection) -> Option<Position> {
        let (row, col) = (position + direction)?;
        (row < self.array.nrows() && col < self.array.ncols()).then_some((row, col))
    }

    fn step_and_shine(&mut self, position: Position, direction: CardinalDirection) {
        if let Some(pos) = self.step(position, direction) {
            self.shine_beam(pos, direction);
        }
    }
}

impl FromStr for Grid {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num_columns = s.lines().next().ok_or(ParseError::EmptyPattern)?.len();
        let locations = s
            .lines()
            .flat_map(str::chars)
            .map(Location::try_from)
            .collect::<Result<Vec<Location>, _>>()?;
        Self::new(num_columns, locations)
    }
}

fn main() -> miette::Result<()> {
    let input = include_str!("../inputs/day_16.txt");
    let grid = Grid::from_str(input)?;
    // println!("{grid}");
    let result = grid.maximize_energized();
    println!("Result: {result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day_16_test_input() {
        let input = include_str!("../inputs/day_16_test.txt");
        let grid = Grid::from_str(input).unwrap();
        let result = grid.maximize_energized();
        assert_eq!(result, 51);
    }

    #[test]
    fn check_day_16_full_input() {
        let input = include_str!("../inputs/day_16.txt");
        let grid = Grid::from_str(input).unwrap();
        let result = grid.maximize_energized();
        assert_eq!(result, 7793);
    }
}
