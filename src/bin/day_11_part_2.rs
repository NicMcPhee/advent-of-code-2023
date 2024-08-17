use std::{
    ops::{Index, IndexMut},
    str::FromStr,
};

use itertools::Itertools;
use miette::Diagnostic;

#[derive(Debug, Clone, Copy)]
enum Axis {
    Row,
    Col,
}

#[derive(Debug, Clone, Copy)]
struct Galaxy {
    row: usize,
    col: usize,
}

impl Index<Axis> for Galaxy {
    type Output = usize;

    fn index(&self, index: Axis) -> &Self::Output {
        match index {
            Axis::Row => &self.row,
            Axis::Col => &self.col,
        }
    }
}

impl IndexMut<Axis> for Galaxy {
    fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
        match index {
            Axis::Row => &mut self.row,
            Axis::Col => &mut self.col,
        }
    }
}

impl Galaxy {
    const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    const fn manhattan_distance(&self, other: &Self) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

#[derive(Debug)]
struct GalaxyMap {
    galaxies: Vec<Galaxy>,
}

const EXPANSION_RATE: usize = 1_000_000;

impl GalaxyMap {
    fn parse_and_adjust(s: &str) -> Result<Self, GalaxyMapError> {
        let galaxy_map: Self = s.parse()?;
        let mut galaxies = galaxy_map.galaxies;

        // Process blank rows
        Self::offset_elements(&mut galaxies, Axis::Row);
        // Process blank columns
        Self::offset_elements(&mut galaxies, Axis::Col);

        Ok(Self { galaxies })
    }

    fn offset_elements(galaxies: &mut [Galaxy], axis: Axis) {
        galaxies.sort_unstable_by_key(|galaxy| galaxy[axis]);
        let mut offset = 0;
        for i in 1..galaxies.len() {
            galaxies[i][axis] += offset;
            let diff = galaxies[i][axis] - galaxies[i - 1][axis];
            if diff > 1 {
                let additional_offset = (diff - 1) * (EXPANSION_RATE - 1);
                offset += additional_offset;
                galaxies[i][axis] += additional_offset;
            }
        }
    }

    fn pairwise_length_sum(&self) -> usize {
        self.galaxies
            .iter()
            .tuple_combinations()
            .map(|(p, q)| p.manhattan_distance(q))
            .sum()
    }
}

#[derive(Debug, thiserror::Error, Diagnostic)]
enum GalaxyMapError {}

impl FromStr for GalaxyMap {
    type Err = GalaxyMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let galaxies = s
            .lines()
            .enumerate()
            .flat_map(|(row_number, row)| {
                row.char_indices().filter_map(move |(col_number, c)| {
                    (c == '#').then_some(Galaxy::new(row_number, col_number))
                })
            })
            .collect::<Vec<Galaxy>>();
        Ok(Self { galaxies })
    }
}

fn main() -> miette::Result<()> {
    let input = include_str!("../inputs/day_11.txt");
    let galaxy_map = GalaxyMap::parse_and_adjust(input)?;
    // println!("{galaxy_map:#?}");
    let result = galaxy_map.pairwise_length_sum();
    println!("Result: {result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_test_input() -> Result<(), GalaxyMapError> {
        let input = include_str!("../inputs/day_11_test.txt");
        let galaxy_map = GalaxyMap::parse_and_adjust(input)?; // .unwrap();
        let result = galaxy_map.pairwise_length_sum(); // .unwrap();
        assert_eq!(result, 82_000_210);
        Ok(())
    }

    #[test]
    fn check_full_input() {
        let input = include_str!("../inputs/day_11.txt");
        let galaxy_map = GalaxyMap::parse_and_adjust(input).unwrap();
        let result = galaxy_map.pairwise_length_sum();
        assert_eq!(result, 707_505_470_642);
    }
}
