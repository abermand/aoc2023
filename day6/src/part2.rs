use lazy_static::lazy_static;
use regex::Regex;
use nom::{
    Parser,
    IResult,
    multi::*,
    bytes::complete::tag,
    character::complete::{self, space1, digit1, newline},
    sequence::{preceded, terminated}
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
struct Table<'a> {
    times: Vec<&'a str>,
    distances: Vec<&'a str>,
}

fn main() {
    let input = include_str!("./sample2.txt");
    let output = do_the_job(input);
    dbg!(output);
}


fn list_of_nums(input: &str) -> IResult<&str, Vec<&str>> {
    //separated_list1(space1, map(digit1, |s: &str| s.parse::<u32>().unwrap())).parse(&input)
    separated_list1(space1, digit1).parse(&input)
}

fn parse_input(input: &str) -> Table{
    let (input, times) : (&str, Vec<&str>) = 
            preceded(
                tag("Time:"),
                preceded(space1,
                    terminated(
                        list_of_nums,
                        newline)
        ))(input).expect("parsing error");
    let (_, distances) : (&str, Vec<&str>) = 
            preceded(
                tag("Distance:"),
                preceded(space1,
                        list_of_nums
        ))(input).expect("parsing error");
    Table{times,distances}
}

fn do_the_job(input: &str) -> u64 {
    let table = parse_input(&input);

    let mut distance = String::new();
        for s in table.distances.into_iter() {
            distance.push_str(s);
        }
    let distance = distance.parse::<u64>().unwrap();
    let mut time = String::new();
        for s in table.times.into_iter() {
            time.push_str(s);
        }
    let time = time.parse::<u64>().unwrap();

    let results = (1..time).into_iter()
                .filter(|t| (time - t) * t > distance);

    let sizes  = results.collect::<Vec<u64>>().len();

    sizes as u64

}


#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./sample1.txt");
        let result = do_the_job(input);
        assert_eq!(result, 71503);
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
