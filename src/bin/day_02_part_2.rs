use pest_consume::{match_nodes, Error, Parser};

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Copy, Clone)]
struct Reveal {
    red: u32,
    green: u32,
    blue: u32,
}

impl Reveal {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }

    fn color_max(self, other: Reveal) -> Reveal {
        Reveal {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
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
                Color::Red => red = red.max(count),
                Color::Green => green = green.max(count),
                Color::Blue => blue = blue.max(count),
            }
        }
        Reveal { red, green, blue }
    }
}

#[derive(Debug)]
struct Game {
    _number: u32,
    reveals: Vec<Reveal>,
}

impl Game {
    fn power(&self) -> u32 {
        self.reveals
            .iter()
            .copied()
            .reduce(Reveal::color_max)
            .unwrap()
            .power()
    }
}

#[derive(Parser)]
#[grammar = "grammars/day_02.pest"]
struct GameParser;

type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

#[pest_consume::parser]
impl GameParser {
    fn input(input: Node) -> Result<Vec<Game>> {
        Ok(match_nodes!(input.into_children();
            [game(g)..] => g.collect(),
        ))
    }

    fn game(input: Node) -> Result<Game> {
        Ok(match_nodes!(input.into_children();
            [int(n), reveal(r)..] => Game { _number: n, reveals: r.collect() },
        ))
    }

    fn int(input: Node) -> Result<u32> {
        Ok(input.as_str().parse().unwrap())
    }

    fn reveal(input: Node) -> Result<Reveal> {
        Ok(match_nodes!(input.into_children();
            [cubeCount(c)..] => c.collect::<Reveal>(),
        ))
    }

    fn cubeCount(input: Node) -> Result<CubeCount> {
        Ok(match_nodes!(input.into_children();
            [int(n), color(c)] => (n, c),
        ))
    }

    fn color(input: Node) -> Result<Color> {
        Ok(match_nodes!(input.into_children();
            [red(c)] => c, [green(c)] => c, [blue(c)] => c,
        ))
    }

    fn red(input: Node) -> Result<Color> {
        Ok(Color::Red)
    }

    fn green(input: Node) -> Result<Color> {
        Ok(Color::Green)
    }

    fn blue(input: Node) -> Result<Color> {
        Ok(Color::Blue)
    }
}

fn sum_of_game_powers(input: &str) -> anyhow::Result<u32> {
    let games = GameParser::parse(Rule::input, input).unwrap();
    let games = games.single()?;
    let games = GameParser::input(games)?;
    Ok(games.iter().map(Game::power).sum())
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../inputs/day_02.txt");
    let result = sum_of_game_powers(input);
    println!("Result: {}", result?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_test_input() {
        let input = include_str!("../inputs/day_02_test.txt");
        let result = sum_of_game_powers(input).unwrap();
        assert_eq!(result, 2286);
    }

    #[test]
    fn check_full_input() {
        let input = include_str!("../inputs/day_02.txt");
        let result = sum_of_game_powers(input).unwrap();
        assert_eq!(result, 77021);
    }
}
