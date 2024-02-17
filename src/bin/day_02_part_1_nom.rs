use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1, u32},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Reveal {
    red: u32,
    green: u32,
    blue: u32,
}

impl Reveal {
    fn within(&self, max_count: &Reveal) -> bool {
        self.red <= max_count.red && self.green <= max_count.green && self.blue <= max_count.blue
    }
}

type CubeCount = (u32, Color);

impl FromIterator<CubeCount> for Reveal {
    fn from_iter<I: IntoIterator<Item = CubeCount>>(iter: I) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for (count, color) in iter {
            match color {
                Color::Red => red += count,
                Color::Green => green += count,
                Color::Blue => blue += count,
            }
        }
        Reveal { red, green, blue }
    }
}

#[derive(Debug)]
struct Game {
    number: u32,
    reveals: Vec<Reveal>,
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    nom::branch::alt((
        nom::combinator::value(Color::Red, tag("red")),
        nom::combinator::value(Color::Green, tag("green")),
        nom::combinator::value(Color::Blue, tag("blue")),
    ))(input)
}

fn parse_cube_count(input: &str) -> IResult<&str, CubeCount> {
    separated_pair(u32, space1, parse_color)(input)
}

fn parse_reveal(input: &str) -> IResult<&str, Reveal> {
    separated_list1(tag(", "), parse_cube_count)(input).map(|(input, counts)| {
        let reveal = counts.into_iter().collect();
        (input, reveal)
    })
}

fn parse_reveals(input: &str) -> IResult<&str, Vec<Reveal>> {
    separated_list1(tag("; "), parse_reveal)(input)
}

fn parse_game_header(input: &str) -> IResult<&str, u32> {
    separated_pair(tag("Game"), space1, u32)(input).map(|(input, (_, number))| (input, number))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, (game_number, reveals)) =
        nom::sequence::separated_pair(parse_game_header, tag(": "), parse_reveals)(input)?;
    Ok((
        input,
        Game {
            number: game_number,
            reveals,
        },
    ))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(newline, parse_game)(input)
}

fn sum_of_legal_game_ids(input: &str) -> anyhow::Result<u32> {
    let max_count = Reveal {
        red: 12,
        green: 13,
        blue: 14,
    };
    let (_, games) =
        all_consuming(parse_games)(input).map_err(nom::Err::<nom::error::Error<&str>>::to_owned)?;
    Ok(games
        .into_iter()
        .filter_map(|game| {
            game.reveals
                .iter()
                .all(|reveal| reveal.within(&max_count))
                .then_some(game.number)
        })
        .sum())
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../inputs/day_02.txt").trim();
    let result = sum_of_legal_game_ids(input);
    println!("Result: {}", result?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_test_input() {
        let input = include_str!("../inputs/day_02_test.txt");
        let result = sum_of_legal_game_ids(input).unwrap();
        assert_eq!(result, 8);
    }

    #[test]
    fn check_full_input() {
        let input = include_str!("../inputs/day_02.txt");
        let result = sum_of_legal_game_ids(input).unwrap();
        assert_eq!(result, 2285);
    }
}
