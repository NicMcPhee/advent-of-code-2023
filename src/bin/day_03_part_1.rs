use std::collections::HashMap;

use pest_consume::{match_nodes, Error, Parser};

#[derive(Debug)]
struct Part {
    number: u32,
    line: usize,
    start: usize,
    end: usize,
}

impl Part {
    fn adjacent_fields(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        // The set of positions above the given `Part`, and extending one to the left and right for
        // diagonal positions.
        let top_line = ((self.start - 1)..=self.end).map(|column| (self.line - 1, column));
        // The set of positions below the given `Part`, and extending one to the left and right for
        // diagonal positions.
        let bottom_line = ((self.start - 1)..=self.end).map(|column| (self.line + 1, column));
        // The set of positions to the left and right of the given `Part`.
        [(self.line, self.start - 1), (self.line, self.end)]
            .into_iter()
            .chain(top_line)
            .chain(bottom_line)
    }
}

#[derive(Debug)]
struct Symbol {
    symbol: char,
    line: usize,
    column: usize,
}

#[derive(Debug)]
enum Cell {
    Part(Part),
    Symbol(Symbol),
}

#[derive(Debug)]
struct Schematic {
    parts: Vec<Part>,
    symbols: HashMap<(usize, usize), char>,
}

impl Schematic {
    fn sum_of_part_numbers(&self) -> u32 {
        self.parts
            .iter()
            .filter(|part| self.has_adjacent_symbol(part))
            .map(|part| part.number)
            .sum()
    }

    fn has_adjacent_symbol(&self, part: &Part) -> bool {
        part.adjacent_fields()
            .any(|(line, column)| self.symbol_at_position(line, column))
    }

    fn symbol_at_position(&self, line: usize, column: usize) -> bool {
        self.symbols.contains_key(&(line, column))
    }
}

impl FromIterator<Cell> for Schematic {
    fn from_iter<I: IntoIterator<Item = Cell>>(iter: I) -> Self {
        let mut parts = Vec::new();
        let mut symbols = HashMap::new();
        for cell in iter {
            match cell {
                Cell::Part(part) => parts.push(part),
                Cell::Symbol(symbol) => {
                    symbols.insert((symbol.line, symbol.column), symbol.symbol);
                }
            }
        }
        Schematic { parts, symbols }
    }
}

#[derive(Parser)]
#[grammar = "grammars/day_03.pest"]
struct SchematicParser;

type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

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
            [symbol(s)] => Cell::Symbol(s),
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

    fn symbol(input: Node) -> Result<Symbol> {
        let symbol = input
            .as_str()
            .chars()
            .next()
            .expect("A symbol must be a single character.");
        let span = input.as_span();
        let (line, column) = span.start_pos().line_col();
        Ok(Symbol {
            symbol,
            line,
            column,
        })
    }
}

fn parse_schematic(input: &str) -> anyhow::Result<Schematic> {
    let parts = SchematicParser::parse(Rule::input, input)?;
    let parts = parts.single()?;
    SchematicParser::input(parts).map_err(Into::into)
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../inputs/day_03.txt");
    let result = parse_schematic(input)?.sum_of_part_numbers();
    println!("Result: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_test_input() {
        let input = include_str!("../inputs/day_03_test.txt");
        let result = parse_schematic(input).unwrap().sum_of_part_numbers();
        assert_eq!(result, 4361);
    }

    #[test]
    fn check_full_input() {
        let input = include_str!("../inputs/day_03.txt");
        let result = parse_schematic(input).unwrap().sum_of_part_numbers();
        assert_eq!(result, 498559);
    }
}
