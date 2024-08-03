use itertools::Itertools;
use pest_consume::{match_nodes, Error, Parser};
use std::collections::HashMap;

trait NextTwo
where
    Self: Iterator,
{
    fn next_two(self) -> Option<(Self::Item, Self::Item)>;
}

impl<T> NextTwo for T
where
    T: Iterator,
{
    fn next_two(mut self) -> Option<(Self::Item, Self::Item)> {
        let first = self.next()?;
        let second = self.next()?;

        if self.next().is_some() {
            return None;
        }

        Some((first, second))
    }
}

type Location = (usize, usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Part {
    number: u32,
    line: usize,
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct Gear {
    line: usize,
    column: usize,
}

impl Gear {
    fn adjacent_fields(&self) -> impl Iterator<Item = Location> + '_ {
        // The set of positions above the given `Gear`, and extending one to the left and right for
        // diagonal positions.
        let top_line =
            ((self.column - 1)..=(self.column + 1)).map(|column| (self.line - 1, column));
        // The set of positions below the given `Gear`, and extending one to the left and right for
        // diagonal positions.
        let bottom_line =
            ((self.column - 1)..=(self.column + 1)).map(|column| (self.line + 1, column));
        // The set of positions to the left and right of the given `Gear`.
        [(self.line, self.column - 1), (self.line, self.column + 1)]
            .into_iter()
            .chain(top_line)
            .chain(bottom_line)
    }
}

#[derive(Debug)]
enum Cell {
    Part(Part),
    Gear(Gear),
}

#[derive(Debug)]
struct Schematic {
    parts: HashMap<Location, Part>,
    gears: Vec<Gear>,
}

impl Schematic {
    fn sum_of_gear_ratios(&self) -> u32 {
        self.gears.iter().filter_map(|gear| self.ratio(gear)).sum()
    }

    fn ratio(&self, gear: &Gear) -> Option<u32> {
        gear.adjacent_fields()
            .filter_map(|location| self.parts.get(&location))
            .unique()
            .map(|part| part.number)
            .next_two()
            .map(|(a, b)| a * b)
    }
}

impl FromIterator<Cell> for Schematic {
    fn from_iter<I: IntoIterator<Item = Cell>>(iter: I) -> Self {
        let mut parts = HashMap::new();
        let mut gears = Vec::new();
        for cell in iter {
            match cell {
                Cell::Part(part) => {
                    parts.insert((part.line, part.start), part);
                    parts.insert((part.line, part.end - 1), part);
                }
                Cell::Gear(gear) => gears.push(gear),
            }
        }
        Self { parts, gears }
    }
}

#[derive(Parser)]
#[grammar = "grammars/day_03_part_2.pest"]
struct SchematicParser;

type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

#[allow(clippy::unnecessary_wraps)]
#[pest_consume::parser]
impl SchematicParser {
    fn input(input: Node) -> Result<Schematic> {
        Ok(match_nodes!(input.into_children();
            [cell(c)..] => c.collect::<Schematic>(),
        ))
    }

    fn cell(input: Node) -> Result<Cell> {
        Ok(match_nodes!(input.into_children();
            [number(p)] => Cell::Part(p),
            [asterisk(g)] => Cell::Gear(g),
        ))
    }

    fn number(input: Node) -> Result<Part> {
        let number = input
            .as_str()
            .parse()
            .expect("A part number must be a valid unsigned integer.");
        let span = input.as_span();
        let (line, start) = span.start_pos().line_col();
        let (_, end) = span.end_pos().line_col();
        Ok(Part {
            number,
            line,
            start,
            end,
        })
    }

    fn asterisk(input: Node) -> Result<Gear> {
        let span = input.as_span();
        let (line, column) = span.start_pos().line_col();
        Ok(Gear { line, column })
    }
}

fn parse_schematic(input: &str) -> anyhow::Result<Schematic> {
    let parts = SchematicParser::parse(Rule::input, input)?;
    let parts = parts.single()?;
    SchematicParser::input(parts).map_err(Into::into)
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../inputs/day_03.txt");
    let result = parse_schematic(input)?.sum_of_gear_ratios();
    println!("Result: {result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_test_input() {
        let input = include_str!("../inputs/day_03_test.txt");
        let result = parse_schematic(input).unwrap().sum_of_gear_ratios();
        assert_eq!(result, 467_835);
    }

    #[test]
    fn check_full_input() {
        let input = include_str!("../inputs/day_03.txt");
        let result = parse_schematic(input).unwrap().sum_of_gear_ratios();
        assert_eq!(result, 72_246_648);
    }
}
