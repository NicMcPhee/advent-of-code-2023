use miette::Diagnostic;
use ndarray::{Array, Array2, Axis, ShapeError};
use std::{fmt::Write, str::FromStr};

#[derive(Debug, Diagnostic, thiserror::Error)]
enum PlatformError {
    #[error("Tried to parse a pattern with no lines")]
    EmptyPattern,

    #[error(transparent)]
    ArrayShape(#[from] ShapeError),

    #[error("Illegal location character {0}")]
    IllegalLocation(char),
}

/// For this to work, Round must come be before Empty in this
/// enum definition, since the sorting in `Platform::roll_lane_forwards()`
/// requires that Round < Empty.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
enum Location {
    Round,
    Cube,
    Empty,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Round => f.write_char('O'),
            Self::Cube => f.write_char('#'),
            Self::Empty => f.write_char('.'),
        }
    }
}

impl Location {
    const fn from_char(c: char) -> Result<Self, PlatformError> {
        Ok(match c {
            '.' => Self::Empty,
            '#' => Self::Cube,
            'O' => Self::Round,
            c => return Err(PlatformError::IllegalLocation(c)),
        })
    }
}

/// Where we're rolling to.
#[derive(Debug, Clone, Copy)]
pub enum CardinalDirection {
    North,
    South,
    East,
    West,
}

impl CardinalDirection {
    const fn axis(self) -> Axis {
        match self {
            Self::North | Self::South => Axis(0),
            Self::East | Self::West => Axis(1),
        }
    }

    const fn lane_direction(self) -> LaneDirection {
        match self {
            Self::North | Self::West => LaneDirection::Forward,
            Self::South | Self::East => LaneDirection::Reversed,
        }
    }
}

enum LaneDirection {
    Forward,
    Reversed,
}

#[derive(Debug)]
struct Platform {
    array: Array2<Location>,
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.array.rows() {
            for location in row {
                location.fmt(f)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Platform {
    fn new(num_columns: usize, locations: Vec<Location>) -> Result<Self, PlatformError> {
        debug_assert_eq!(locations.len() % num_columns, 0);
        let num_rows = locations.len() / num_columns;
        let array = Array::from_shape_vec((num_rows, num_columns), locations)?;
        Ok(Self { array })
    }

    fn total_load(&self, direction: CardinalDirection) -> Result<usize, PlatformError> {
        let platform_after_rolling = self.roll(direction)?;
        // println!("{platform_after_rolling}");
        Ok(platform_after_rolling
            .array
            .lanes(Axis(1))
            .into_iter()
            .map(Self::lane_load)
            .sum())
    }

    fn lane_load<'a>(
        lane: impl IntoIterator<Item = &'a Location, IntoIter: DoubleEndedIterator>,
    ) -> usize {
        lane.into_iter()
            .rev()
            .enumerate()
            .filter_map(|(position, location)| {
                (location == &Location::Round).then_some(position + 1)
            })
            .sum()
    }

    fn roll(&self, direction: CardinalDirection) -> Result<Self, PlatformError> {
        let locations: Vec<Location> = self
            .array
            .lanes(direction.axis())
            .into_iter()
            .flat_map(|lane| Self::roll_lane(lane, &direction.lane_direction()))
            .collect();
        Self::new(self.num_lanes_in_direction(direction), locations)
    }

    fn roll_lane<'a>(
        lane: impl IntoIterator<Item = &'a Location, IntoIter: DoubleEndedIterator>,
        lane_direction: &LaneDirection,
    ) -> Vec<Location> {
        match lane_direction {
            LaneDirection::Forward => Self::roll_lane_forwards(lane),
            LaneDirection::Reversed => Self::roll_lane_forwards(lane.into_iter().rev()),
        }
    }

    fn roll_lane_forwards<'a>(locations: impl IntoIterator<Item = &'a Location>) -> Vec<Location> {
        let mut locations = locations.into_iter().copied().collect::<Vec<_>>();
        locations
            .split_mut(|location| location == &Location::Cube)
            .for_each(<[Location]>::sort_unstable);
        locations
    }

    fn num_lanes_in_direction(&self, direction: CardinalDirection) -> usize {
        self.array.lanes(direction.axis()).into_iter().len()
    }
}

impl FromStr for Platform {
    type Err = PlatformError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num_columns = s.lines().next().ok_or(PlatformError::EmptyPattern)?.len();
        let locations = s
            .lines()
            .flat_map(|s| s.chars().map(Location::from_char))
            .collect::<Result<Vec<Location>, _>>()?;
        Self::new(num_columns, locations)
    }
}

fn main() -> miette::Result<()> {
    let input = include_str!("../inputs/day_14.txt");
    let platform = Platform::from_str(input)?;
    println!("{platform:#?}");
    let result = platform.total_load(CardinalDirection::North)?;
    println!("Result: {result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day_14_test_input() {
        let input = include_str!("../inputs/day_14_test.txt");
        let platform = Platform::from_str(input).unwrap();
        let result = platform.total_load(CardinalDirection::North).unwrap();
        assert_eq!(result, 136);
    }

    #[test]
    fn check_day_14_full_input() {
        let input = include_str!("../inputs/day_14.txt");
        let platform = Platform::from_str(input).unwrap();
        let result = platform.total_load(CardinalDirection::North).unwrap();
        assert_eq!(result, 109_755);
    }
}
