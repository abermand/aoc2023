use lazy_static::lazy_static;
use regex::Regex;
use nom::{
    IResult,
    multi::*,
    bytes::complete::tag,
    character::complete::alphanumeric1,
    sequence::delimited,
};
use num::Integer;

use std::collections::HashMap;

lazy_static! {
    static ref NUMBERS_REGEX : Regex = Regex::new(r"(\d+)").unwrap();
}

#[derive(Debug, Clone, Copy)]
struct Map<'a> {
    key: &'a str, 
    left: &'a str, 
    right: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
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

fn parse_list(list: &str) -> IResult<&str, Vec<&str>> {
        separated_list1(tag(", "), alphanumeric1)(list)
}

fn is_start(s: &str) -> bool {
    s.chars().last().unwrap() == 'A'
}

fn is_end(s: &str) -> bool {
    s.chars().last().unwrap() == 'Z'
}

fn parse_map_line(line: &str) -> Map {
    let (key, therest) = line.split_once(" = ").unwrap();

    let (_input, directions) : (&str, Vec<&str>) = delimited(
        tag("("),
        parse_list,
        tag(")")
        ) (therest).expect("parsing error");
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
    let hashmap : HashMap<&str, Map> = maps.iter().map(|m| (m.key, *m))
        .collect();
    (pattern, hashmap)
}

fn do_the_job(input: &str) -> u64 {
    let (pattern, maps) = parse_input(&input);
    let mut counts: Vec<u64> = vec![];

    for key in maps.keys().filter(|x| is_start(x)) {
        let mut state : &str = key;
        let mut rounds : u64 = 1;
        for dir in pattern.iter().cycle() {
            let next;
            let map = maps.get(state).unwrap();
            if *dir == Direction::Left {
                next = map.left;
            } else {
                next = map.right;
            }
            // println!("{} #{} {:?} : {} -> {}", key, &rounds, dir, from, &next);
            if is_end(next) {
                break;
            }
            state = next;
            rounds += 1;
        }
        counts.push(rounds);
    }
    dbg!(&counts);
    let mut counts_lcm : u64 = 1;
    for count in counts.iter() {
        counts_lcm = counts_lcm.lcm(&count) as u64;
    }
    counts_lcm
}


#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./sample3.txt");
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
