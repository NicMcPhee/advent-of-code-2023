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

#[derive(Debug)]
struct Map<'a> {
    path: Vec<Direction>,
    connections: Vec<Connection<'a>>,
}

fn parser<'a>() -> impl Parser<'a, &'a str, Map<'a>> {
    let path = path();

    let connection = parse_connection();

    path.padded()
        .then(
            connection
                .separated_by(newline())
                .at_least(1)
                .collect::<Vec<_>>(),
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
        .filter(|c: &char| c.is_ascii_uppercase())
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

impl Map<'_> {
    fn num_steps(&self) -> u32 {
        todo!()
    }
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../inputs/day_08_test_1.txt");

    let map = parser().parse(input).into_result().map_err(|parse_errs| {
        for e in parse_errs {
            println!("Parse error: {e:#?}");
        }
        anyhow::anyhow!("Parsing error")
    })?;

    dbg!(&map);
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
        let input = include_str!("../inputs/day_08_test_1.txt");
        let map = parser().parse(input).into_result().unwrap();
        let result = map.num_steps();
        assert_eq!(result, 2);
    }

    #[test]
    fn check_test_input_2() {
        let input = include_str!("../inputs/day_08_test_2.txt");
        let map = parser().parse(input).into_result().unwrap();
        let result = map.num_steps();
        assert_eq!(result, 6);
    }

    #[test]
    fn check_full_input() {
        let input = include_str!("../inputs/day_08.txt");
        let map = parser().parse(input).into_result().unwrap();
        let result = map.num_steps();
        assert_eq!(result, 251_195_607);
    }
}
