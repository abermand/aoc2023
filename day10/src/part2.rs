use lazy_static::lazy_static;
use regex::Regex;
// use nom::{
//     Parser,
//     IResult,
//     multi::*,
//     bytes::complete::tag,
//     character::complete::{self, space1, newline},
//     sequence::{preceded, terminated}
// };
//use nom_supreme::{tag::complete::tag, ParserExt};

use std::cmp;
// use std::collections::HashMap;
//use rstest::*;

lazy_static! {
    static ref NUMBERS_REGEX : Regex = Regex::new(r"(\d+)").unwrap();
    //static ref SYMBOLS_REGEX : Regex = Regex::new(r"[^0-9.]").unwrap();
    //static ref MAP_REGEX: Regex = Regex::new(r"(\S+)-to-(\S+) map:").unwrap();
}

#[derive(Debug)]
struct Struct {
    }

fn main() {
    let input = include_str!("./sample2.txt");
    let output = do_the_job(input);
    dbg!(output);
}


fn parse_input(input: &str) -> Struct {
    Struct {}
}

fn do_the_job(input: &str) -> u32 {
    let _s = parse_input(&input);
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
        assert_eq!(result, 456);
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
