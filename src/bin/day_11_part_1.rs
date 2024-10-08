use std::str::FromStr;

use itertools::Itertools;
use miette::Diagnostic;

#[derive(Debug)]
struct Galaxy {
    row: usize,
    col: usize,
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

impl GalaxyMap {
    fn parse_and_adjust(s: &str) -> Result<Self, GalaxyMapError> {
        let galaxy_map: Self = s.parse()?;
        let mut galaxies = galaxy_map.galaxies;

        // Sort galaxies by x-coordinate
        galaxies.sort_unstable_by_key(|galaxy| galaxy.row);
        let mut offset = 0;
        for i in 1..galaxies.len() {
            galaxies[i].row += offset;
            let diff = galaxies[i].row - galaxies[i - 1].row;
            if diff > 1 {
                offset += diff - 1;
                galaxies[i].row += diff - 1;
            }
        }

        // Sort galaxies by y-coordinate
        galaxies.sort_unstable_by_key(|galaxy| galaxy.col);
        let mut offset = 0;
        for i in 1..galaxies.len() {
            galaxies[i].col += offset;
            let diff = galaxies[i].col - galaxies[i - 1].col;
            if diff > 1 {
                offset += diff - 1;
                galaxies[i].col += diff - 1;
            }
        }

        Ok(Self { galaxies })
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
        assert_eq!(result, 374);
        Ok(())
    }

    #[test]
    fn check_full_input() {
        let input = include_str!("../inputs/day_11.txt");
        let galaxy_map = GalaxyMap::parse_and_adjust(input).unwrap();
        let result = galaxy_map.pairwise_length_sum();
        assert_eq!(result, 10_885_634);
    }
}
