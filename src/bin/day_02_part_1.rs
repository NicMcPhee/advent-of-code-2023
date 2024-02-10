use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammars/day_02.pest"]
struct GameParser;

fn sum_of_legal_game_ids(input: &str) -> u32 {
    let mut result = 0;
    let max_count = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let games = GameParser::parse(Rule::input, input).unwrap();
    for game in games {
        let Rule::game = game.as_rule() else {
            panic!("Expected a game, but got {:?}", game);
        };
        let mut game_components = game.into_inner();
        let game_number: u32 = game_components.next().unwrap().as_str().parse().unwrap();
        let mut is_legal = true;
        for reveal in game_components {
            for cube_count_pair in reveal.into_inner() {
                let mut cube_count_iter = cube_count_pair.into_inner();
                let count: u32 = cube_count_iter.next().unwrap().as_str().parse().unwrap();
                let color = cube_count_iter.next().unwrap().as_str();
                if count > *max_count.get(color).unwrap() {
                    is_legal = false;
                    break;
                }
            }
            if !is_legal {
                break;
            }
        }
        if is_legal {
            result += game_number;
        }
    }

    result
}

fn main() {
    let input = include_str!("../inputs/day_02.txt");

    let result = sum_of_legal_game_ids(input);

    println!("Result: {}", result);
}
