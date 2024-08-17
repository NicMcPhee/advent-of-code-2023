use std::str::FromStr;

use miette::Diagnostic;

#[derive(Debug)]
struct Galaxy {
    x: usize,
    y: usize,
}

impl Galaxy {
    fn new(x: usize, y: usize) -> Self {
        Galaxy { x, y }
    }
}

#[derive(Debug)]
struct GalaxyMap {
    galaxies: Vec<Galaxy>,
}

impl GalaxyMap {
    fn pairwise_length_sum(&self) -> i64 {
        todo!()
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
    let input = include_str!("../inputs/day_11_test.txt");
    let galaxy_map = GalaxyMap::from_str(input)?;
    println!("{galaxy_map:#?}");
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
        let galaxy_map = GalaxyMap::from_str(input)?; // .unwrap();
        let result = galaxy_map.pairwise_length_sum(); // .unwrap();
        assert_eq!(result, 4);
        Ok(())
    }

    #[test]
    fn check_full_input() {
        let input = include_str!("../inputs/day_11.txt");
        let galaxy_map = GalaxyMap::from_str(input).unwrap();
        let result = galaxy_map.pairwise_length_sum();
        assert_eq!(result, 6886);
    }
}
