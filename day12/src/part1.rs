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

use rstest::*;

lazy_static! {
    static ref NUMBERS_REGEX : Regex = Regex::new(r"(\d+)").unwrap();
    //static ref SYMBOLS_REGEX : Regex = Regex::new(r"[^0-9.]").unwrap();
    //static ref MAP_REGEX: Regex = Regex::new(r"(\S+)-to-(\S+) map:").unwrap();
}

#[derive(Debug)]
enum State {
    Works,
    Faulty,
    Unknown
}

#[derive(Debug)] 
struct LineMap {
    states: Vec<State>,
    faults: Vec<u64>,
}

fn main() {
    let input = include_str!("./sample2.txt");
    let output = do_the_job(input);
    dbg!(output);
}


fn parse_input(input: &str) -> Vec<LineMap> {
    let mut maps : Vec<LineMap> = vec![];
    
    for line in input.lines() {        
        let (left, right) = line.split_once(" ").unwrap();
        let states : Vec<State>= left.chars().map(|c| match c {
            '?' => State::Unknown,
            '#' => State::Faulty,
            _ => State::Works,
        }).collect();
        let faults : Vec<u64> = right.split(",").map(|x| x.parse::<u64>().unwrap()).collect();

        maps.push(LineMap{states, faults});
    }
    maps
}

fn reduce(states: &[State], faults: &[u64], expected: u64, indent: usize) -> u64 {
    let pad = " ".repeat(2*indent);
    println!("{}Reduce S={:?}, F={:?} e={}", pad, &states, &faults, &expected);
    if states.len() == 0 {
        return 0;
    }
    match states[0] {
        State::Works => {
            if expected > 0 {
                return 0;
            } else {
                if faults.len() == 0 {
                    return 1;
                }
                return reduce(&states[1..], faults, 0, indent+1);
            }
        },
        State::Faulty => {
            if expected > 0 {
                return reduce(&states[1..], faults, expected - 1, indent+1)
            }
            if faults.len() == 0 {
                return 0;
            }
            let expected = faults[0];
            reduce(&states[1..], &faults[1..], expected-1, indent+1)
        },
        State::Unknown => {
            if expected > 0 { // in that case it can only be faulty, so look forward 
                return reduce(&states[1..], faults, expected - 1, indent+1);
            }
            if faults.len() == 0 { // in that case it can only be working and we have only one choice
                return 1;
            }
            // otherwise, we have two choices:
            println!("{}Two choices here ------", pad);
            let choice_working = reduce(&states[1..], faults, 0, indent+1);
            let choice_faulty = reduce(&states[1..], &faults[1..], faults[0] - 1, indent+1);
            println!("{}------ returning: {} + {}", pad, &choice_working, &choice_faulty);
            return choice_faulty + choice_working;
        }
    }
}

fn do_the_job(input: &str) -> u64 {
    let s = parse_input(&input); 
    let result : u64 = s.iter().map(|map| {
        println!("\n---- LINE ----\n");
        reduce(&map.states, &map.faults, 0,0)
    }).sum();
    result
}


#[cfg(test)] 
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let input = include_str!("./sample1.txt");
    //     let result = do_the_job(input);
    //     assert_eq!(result, 21);
    // }

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    // #[case(".??..??...?##. 1,1,3", 4)]
    // #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    // #[case("????.#...#... 4,1,1", 1)]
    // #[case("????.######..#####. 1,6,5", 4)]
    // #[case("?###???????? 3,2,1", 10)]
    fn test_expand(#[case] line: &str, #[case] expected: u64) {
        let result = do_the_job(line);
        assert_eq!(result, expected);
    }
}
