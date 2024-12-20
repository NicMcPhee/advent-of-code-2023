use miette::Diagnostic;
use ndarray::{Array, Array2, Axis, ShapeError};
use std::{collections::HashMap, fmt::Write, hash::Hash, str::FromStr};

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
#[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

    fn total_load_after_cycles(self, num_cycles: usize) -> Result<usize, PlatformError> {
        let mut seen_platforms: HashMap<Self, usize> = HashMap::new();
        let mut platform = self;
        seen_platforms.insert(platform.clone(), 0);

        let mut remaining_cycles = num_cycles;
        let final_platform = loop {
            if remaining_cycles == 0 {
                break platform;
            }
            platform = platform
                .roll(CardinalDirection::North)?
                .roll(CardinalDirection::West)?
                .roll(CardinalDirection::South)?
                .roll(CardinalDirection::East)?;
            remaining_cycles -= 1;
            if let Some(&remaining_cycles_at_loop_start) = seen_platforms.get(&platform) {
                seen_platforms.clear();
                remaining_cycles %= remaining_cycles_at_loop_start - remaining_cycles;
            } else {
                seen_platforms.insert(platform.clone(), remaining_cycles);
            }
        };
        // println!("Final platform\n{final_platform}");
        Ok(final_platform.compute_load())
    }

    fn compute_load(&self) -> usize {
        self.array
            .lanes(Axis(0))
            .into_iter()
            .map(Self::lane_load)
            .sum()
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
        let mut result = Self::new(self.num_lanes_in_direction(direction), locations)?;
        match direction {
            CardinalDirection::North => result.array.swap_axes(0, 1),
            CardinalDirection::South => {
                result.array.swap_axes(0, 1);
                result.array.invert_axis(Axis(0));
            }
            CardinalDirection::East => result.array.invert_axis(Axis(1)),
            CardinalDirection::West => {}
        }
        Ok(result)
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

const NUM_CYCLES: usize = 1_000_000_000;

fn main() -> miette::Result<()> {
    let input = include_str!("../inputs/day_14.txt");
    let platform = Platform::from_str(input)?;
    // println!("{platform:#?}");
    let result = platform.total_load_after_cycles(NUM_CYCLES)?;
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
        let result = platform.total_load_after_cycles(NUM_CYCLES).unwrap();
        assert_eq!(result, 64);
    }

    #[test]
    fn check_day_14_full_input() {
        let input = include_str!("../inputs/day_14.txt");
        let platform = Platform::from_str(input).unwrap();
        let result = platform.total_load_after_cycles(NUM_CYCLES).unwrap();
        assert_eq!(result, 90928);
    }
}
