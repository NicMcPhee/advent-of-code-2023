use std::str::Lines;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammars/day_02.pest"]
struct GameParser;

// Returns the game number if this game is legal
fn legal_game(line: &str) -> Option<u32> {
    let game = GameParser::parse(Rule::game, line).unwrap();
    todo!()
}

fn count_legal_games(lines: Lines) -> u32 {
    lines.filter_map(legal_game).sum::<u32>()
}

fn main() {
    // Read the input file "day_01_test.txt"
    // and store it in the variable "input"
    let input = include_str!("../inputs/day_02_test.txt");
    let lines = input.lines();

    let result = count_legal_games(lines);

    println!("Result: {}", result);
}
