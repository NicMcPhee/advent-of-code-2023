use std::collections::HashMap;

use chumsky::prelude::*;
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
    fn advance_node(&self, node: &mut &'a str, direction: Direction) {
        let Some(connection) = self.connections.get(node) else {
            panic!(
                "Failed to find node {node} in the connections map: {:#?}",
                self.connections
            )
        };
        *node = connection.step(direction);
    }

    fn next_nodes(&self, nodes: &mut [&'a str], direction: Direction) -> Option<Vec<&'a str>> {
        nodes
            .iter_mut()
            .for_each(|n| self.advance_node(n, direction));
        nodes
            .iter()
            .any(|n| !n.ends_with('Z'))
            .then(|| nodes.to_vec())
    }

    fn num_steps(&self) -> usize {
        let start_nodes = self
            .connections
            .keys()
            .copied()
            .filter(|k| k.ends_with('A'))
            .collect::<Vec<_>>();
        let steps = self.path.iter().copied().cycle();
        let visited_nodes = steps.scan(
            start_nodes,
            |current_nodes: &mut Vec<&'a str>, direction: Direction| {
                self.next_nodes(current_nodes, direction)
            },
        );
        visited_nodes.count() + 1
    }
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../inputs/day_08.txt");

    let map = parser().parse(input).into_result().map_err(|parse_errs| {
        for e in parse_errs {
            println!("Parse error: {e:#?}");
        }
        anyhow::anyhow!("Parsing error")
    })?;

    // dbg!(&map);

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
            .parse("AAA = (BBB, CCC)\nBBB = (DDD, EEE)")
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
        assert_eq!(result, 21_409);
    }
}
