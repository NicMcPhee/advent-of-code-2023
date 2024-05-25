use chumsky::prelude::*;

struct Map<'a> {
    path: &'a str,
}

fn parser<'a>() -> impl Parser<'a, &'a str, Map<'a>> {
    todo!()
}

impl Map<'_> {
    fn num_steps(&self) -> u32 {
        todo!()
    }
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../inputs/day_08_test_1.txt");

    match parser().parse(&input).into_result() {
        Ok(map) => {
            let result = map.num_steps();
            println!("Result: {result}");
        }
        Err(parse_errs) => parse_errs
            .into_iter()
            .for_each(|e| println!("Parse error: {}", e)),
    };

    Ok(())
}

#[cfg(test)]
mod day_08_part_1_tests {
    use super::*;

    #[test]
    fn check_test_input_1() {
        let input = include_str!("../inputs/day_08_test_1.txt");
        let mut map = Map::from_str(input).unwrap();
        let result = map.num_steps();
        assert_eq!(result, 2);
    }

    #[test]
    fn check_test_input_2() {
        let input = include_str!("../inputs/day_08_test_2.txt");
        let mut map = Map::from_str(input).unwrap();
        let result = map.num_steps();
        assert_eq!(result, 6);
    }

    #[test]
    fn check_full_input() {
        let input = include_str!("../inputs/day_08.txt");
        let mut map = Map::from_str(input).unwrap();
        let result = map.num_steps();
        assert_eq!(result, 251_195_607);
    }
}
