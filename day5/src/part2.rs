use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use core::str::FromStr;
//use rstest::*;

lazy_static! {
    static ref NUMBERS_REGEX : Regex = Regex::new(r"(\d+)").unwrap();
    //static ref SYMBOLS_REGEX : Regex = Regex::new(r"[^0-9.]").unwrap();
    static ref MAP_REGEX: Regex = Regex::new(r"(\S+)-to-(\S+) map:").unwrap();
}

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
enum State {
    Seeds,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}
impl FromStr for State {

    type Err = String;

    fn from_str(input: &str) -> Result<State, Self::Err> {
        match input {
            "seed" => Ok(State::Seeds),
            "soil" => Ok(State::Soil),
            "fertilizer" => Ok(State::Fertilizer),
            "water" => Ok(State::Water),
            "light" => Ok(State::Light),
            "temperature" => Ok(State::Temperature),
            "humidity" => Ok(State::Humidity),
            "location" => Ok(State::Location),
            x      => Err(format!("Could not parse state {}", x)),
        }
    }
}

#[derive(Debug)]
struct Translation {
    from: u64,
    to: u64,
    len: u64,
}

#[derive(Debug)]
struct Map {
    src: State,
    dest: State,
    translations: Vec<Translation>,
}

impl Map {
    fn parse(section: &str) -> Map {
        let mut lines = section.lines();
        let header = lines.next().unwrap();

        let caps = MAP_REGEX.captures(header).unwrap();

        let src = State::from_str(caps.get(1).unwrap().as_str()).unwrap();
        let dest = State::from_str(caps.get(2).unwrap().as_str()).unwrap();
        let mut translations : Vec<Translation> =vec![];

        for line in lines {
            let mut descr = line.split(" ")
            .map(|x| x.parse::<u64>().unwrap());
            let to = descr.next().unwrap();
            let from = descr.next().unwrap();
            let len = descr.next().unwrap();
            translations.push(Translation{from, to, len});
        }

        let map = Map {
            src,
            dest,
            translations,
        };

        // dbg!(&map);
        map
    }
}

fn main() {
    let input = include_str!("./sample2.txt");
    let output = do_the_job(input);
    dbg!(output);
}

fn do_the_job(input: &str) -> u64 {
    let mut sections = input.split("\n\n");
    let mut result : u64 = 0; 


    let header = sections.next().unwrap();
    // dbg!(&header);
    let mut seeds : Vec<u64> = vec![];
    let mut seeds_pairs = header
        .split_once("seeds: ")
        .unwrap().1
        .split(" ")
        .into_iter()
        .map(|x| x.parse::<u64>().unwrap())
        .into_iter()
        .peekable();
    //let mut iter = seeds_pairs.take(2);
    while seeds_pairs.peek().is_some() {
        let x = seeds_pairs.next().unwrap();
        let l = seeds_pairs.next().unwrap();
        let v : Vec<u64> = (x..x+l).step_by(1).collect();
        seeds.extend_from_slice(&v);
    }
        
    // dbg!(&seeds);

    let mut maps : HashMap<State,Map> = HashMap::new();

    for section in sections {
        let map = Map::parse(section);
        maps.insert(map.src.clone(), map);
    }

    let mut min_location = u64::MAX; 
    for seed in seeds {
        let mut state : State = State::Seeds;
        let mut cur = seed;
        while state != State::Location {
        let map = maps.get(&state).unwrap();
            if let Some(trn) = map.translations
                .iter()
                .find(|t| cur >= t.from && cur < (t.from + t.len)) {
                    cur = trn.to + (cur - trn.from);
                    //println!("#{:>10} {:>12} -> {:>12} :  found {:>10}", seed, map.src, map.dest, cur);
                    } else {
                    //println!("#{:>10} {:>12} -> {:>12} : !found {:>10}", seed, map.src, map.dest, cur);
                }
            state = map.dest;
        }
        min_location = cmp::min(min_location, cur);
    }

    result = min_location;
    result
}


#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./sample1.txt");
        let result = do_the_job(input);
        assert_eq!(result, 46);
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
