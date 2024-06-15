use std::collections::HashMap;

use chumsky::prelude::*;
use num::Integer;
use text::newline;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq)]
struct Connection<'a> {
    node_name: &'a str,
    left: &'a str,
    right: &'a str,
}

impl<'a> Connection<'a> {
    const fn step(&self, direction: Direction) -> &'a str {
        match direction {
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
}

#[derive(Debug)]
struct Map<'a> {
    path: Vec<Direction>,
    connections: HashMap<&'a str, Connection<'a>>,
}

fn parser<'a>() -> impl Parser<'a, &'a str, Map<'a>> {
    let path = path();

    let connection = parse_connection();

    path.padded()
        .then(
            connection
                .map(|c| (c.node_name, c))
                .separated_by(newline())
                .at_least(1)
                .collect::<HashMap<_, _>>(),
        )
        .padded()
        .map(|(path, connections)| Map { path, connections })
}

fn parse_connection<'a>() -> impl Parser<'a, &'a str, Connection<'a>> {
    let connections = parse_name().then_ignore(just(',')).then(parse_name());
    (parse_name())
        .then_ignore(just('=').padded())
        .then(connections.delimited_by(just('('), just(')')))
        .map(|(node_name, (left, right))| Connection {
            node_name,
            left,
            right,
        })
}

fn parse_name<'a>() -> impl Parser<'a, &'a str, &'a str> {
    any()
        .filter(|c: &char| c.is_alphanumeric())
        .repeated()
        .exactly(3)
        .to_slice()
        .padded()
}

fn path<'a>() -> impl Parser<'a, &'a str, Vec<Direction>> {
    choice((
        just('L').to(Direction::Left),
        just('R').to(Direction::Right),
    ))
    .repeated()
    .collect::<Vec<_>>()
    .padded()
}

impl<'a> Map<'a> {
    fn advance_node(&self, node: &'a str, direction: Direction) -> &'a str {
        let Some(connection) = self.connections.get(node) else {
            panic!(
                "Failed to find node {node} in the connections map: {:#?}",
                self.connections
            )
        };
        connection.step(direction)
    }

    // Returns (num steps to first occurance, cycle length)
    fn cycle_length(&self, starting_node: &str) -> (usize, usize) {
        type PathIndex = usize;
        type StepCount = usize;

        // (node, LR chain position) -> total step count
        let mut visited_nodes: HashMap<(&str, PathIndex), StepCount> = HashMap::new();
        // An "infinite" iterator over the path steps, repeated indefinitely. Clever uses
        // of `.enumerate()` (thanks to @MizardX) lead to automagic numbering of the steps,
        // where the "outer" number is the total number of steps to that point, and the
        // "inner" number is the current index into the input path.
        let steps = self.path.iter().copied().enumerate().cycle().enumerate();

        let mut current_node = starting_node;
        visited_nodes.insert((current_node, 0), 0);

        for (step_count, (path_index, direction)) in steps {
            current_node = self.advance_node(current_node, direction);
            // We only care about storing "end" nodes in the map, and can ignore all the
            // other nodes (except for the need to count them in path lengths).
            if current_node.ends_with('Z') {
                if let Some(initial_steps_to_node) = visited_nodes.get(&(current_node, path_index))
                {
                    // If we've seen this node/path index pair then we've found a cycle!
                    let cycle_length = step_count - initial_steps_to_node;
                    return (*initial_steps_to_node + 1, cycle_length);
                }
                println!("From {starting_node} reached {current_node} with path index {path_index} and step count {step_count}.");
                visited_nodes.insert((current_node, path_index), step_count);
            }
        }
        unreachable!("The loop above is infinite and should exit via the `return` statement.");
    }

    fn num_steps(&self) -> usize {
        let starting_points: Vec<(&&str, &Connection)> = self
            .connections
            .iter()
            .filter(|c| c.0.ends_with('A'))
            .collect::<Vec<_>>();

        // (num steps to first occurrence, cycle length)
        let cycle_lengths: Vec<(usize, usize)> = starting_points
            .iter()
            .map(|s| self.cycle_length(s.0))
            .collect::<Vec<_>>();

        println!("Cycle lengths = {cycle_lengths:#?}");

        let result = cycle_lengths
            .iter()
            .map(|(_, cl)| *cl)
            .reduce(|acc, cl| acc.lcm(&cl))
            .unwrap();
        result
    }
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../inputs/day_08.txt");

    let map: Map = parser().parse(input).into_result().map_err(|parse_errs| {
        for e in parse_errs {
            println!("Parse error: {e:#?}");
        }
        anyhow::anyhow!("Parsing error")
    })?;

    let result = map.num_steps();

    println!("Result: {result}");

    Ok(())
}

#[cfg(test)]
mod parsing_tests {
    use super::*;

    #[test]
    fn test_path() {
        let path = path()
            .parse("RL\n\n")
            .into_result()
            .expect("Failed to parse path");
        assert_eq!(path, [Direction::Right, Direction::Left]);
    }

    #[test]
    fn test_name() {
        let name = parse_name()
            .parse(" XYZ ")
            .into_result()
            .expect("Failed to parse name");
        assert_eq!(name, "XYZ");
    }

    #[test]
    fn test_connection() {
        let connection = parse_connection()
            .parse("AAA = (BBB, CCC)")
            .into_result()
            .expect("Failed to parse connection");
        assert_eq!(
            connection,
            Connection {
                node_name: "AAA",
                left: "BBB",
                right: "CCC"
            }
        );
        println!("{}", std::any::type_name_of_val(&connection));
    }
}

#[cfg(test)]
mod day_08_part_1_tests {
    use super::*;

    #[test]
    fn check_test_input_1() {
        let input = include_str!("../inputs/day_08_test_3.txt");
        let map = parser().parse(input).into_result().unwrap();
        let result = map.num_steps();
        assert_eq!(result, 6);
    }

    #[test]
    fn check_full_input() {
        let input = include_str!("../inputs/day_08.txt");
        let map = parser().parse(input).into_result().unwrap();
        let result = map.num_steps();
        assert_eq!(result, 21_165_830_176_709);
    }
}
