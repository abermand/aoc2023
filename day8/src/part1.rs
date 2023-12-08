use lazy_static::lazy_static;
use regex::Regex;
use nom::{
    Parser,
    IResult,
    multi::*,
    bytes::complete::tag,
    character::complete::{self, alpha1 },
    sequence::{delimited, preceded, terminated, separated_pair},
    combinator::{cut,rest},
};
//use nom_supreme::{tag::complete::tag, ParserExt};

use std::cmp;
use std::collections::HashMap;
//use rstest::*;

lazy_static! {
    static ref NUMBERS_REGEX : Regex = Regex::new(r"(\d+)").unwrap();
    //static ref SYMBOLS_REGEX : Regex = Regex::new(r"[^0-9.]").unwrap();
    //static ref MAP_REGEX: Regex = Regex::new(r"(\S+)-to-(\S+) map:").unwrap();
}

#[derive(Debug)]
struct Map<'a> {
    key: &'a str, 
    left: &'a str, 
    right: &'a str,
}

#[derive(Debug)]
enum Direction {
    Left, 
    Right,
}

type Pattern = Vec<Direction>;

fn main() {
    let input = include_str!("./sample2.txt");
    let output = do_the_job(input);
    dbg!(output);
}

fn parse_map_line(line: &str) -> Map {
    let (_input, (key, therest)) = separated_pair(cut(alpha1), tag(" = "), rest)(line).expect("parsing_error");
    let (_input, directions) = delimited(
        tag("("),
        separated_list1(tag(", "), alpha1),
        tag(")")
        ) (the).expect("parsing error");
    let left = directions[0];
    let right = directions[1];
    println!("New map: k={} l={} r={}", &key, &left, &right);
    Map{key,left,right}
}

fn parse_input(input: &str) -> (Pattern , HashMap<&str, Map>) {
    let (header, rest) = input.split_once("\n\n").unwrap();
    let pattern : Pattern = header.chars().map(|c| if c == 'L' { Direction::Left } else { Direction::Right }).collect();
    println!("Pattern: {:?}", &pattern);
    let maps : Vec<Map> = rest.lines().map(parse_map_line).collect();
    let maps : HashMap<&str, Map> = maps.map(|m| (m.key.clone(), m))
        .collect::<HashMap<_,_>>();
    (pattern, maps)
}

fn do_the_job(input: &str) -> u32 {
    let (_pattern, _map) = parse_input(&input);
    todo!();
    123
}


#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./sample1.txt");
        let result = do_the_job(input);
        assert_eq!(result, 6);
    }

    //    #[rstest]
    //   #[case("Game 95: 1 blue, 11 red; 15 red, 1 blue, 3 green; 13 red, 2 blue, 3 green; 1 green, 1 blue", false)]
    //#[case("Game 98: 8 red, 12 green, 2 blue; 7 green, 8 red, 1 blue; 2 blue, 6 red, 3 green; 9 red, 1 blue, 4 green", true)]
    //#[case("Game 5: 17 red, 5 blue, 3 green; 8 green, 9 red, 10 blue; 2 green, 9 blue, 4 red", false)]
    //fn is_game_possible(#[case] line: &str, #[case] expected: bool) {
    //let game = Game::parse_line(line);
    //assert_eq!(game.is_possible(), expected);
    //}
}
