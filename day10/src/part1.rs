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

use std::{cmp, collections::HashMap};
// use std::collections::HashMap;
//use rstest::*;

lazy_static! {
    static ref NUMBERS_REGEX : Regex = Regex::new(r"(\d+)").unwrap();
    //static ref SYMBOLS_REGEX : Regex = Regex::new(r"[^0-9.]").unwrap();
    //static ref MAP_REGEX: Regex = Regex::new(r"(\S+)-to-(\S+) map:").unwrap();
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn reverse(self: &Self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

const START : char = 'S';

fn tag_to_pipe(c: char) -> Option<(Direction, Direction)> {
    match c {
        '|' => Some((Direction::North, Direction::South)),
        '-' => Some((Direction::West,  Direction::East)),
        'F' => Some((Direction::South, Direction::East)),
        'J' => Some((Direction::North, Direction::West)),
        'L' => Some((Direction::North, Direction::East)),
        '7' => Some((Direction::South, Direction::West)),
        _ => None,
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coord {
    x: usize, 
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Step {
    dir: Direction,
    to: Coord,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum NextSteps {
    None,
    One(Step),
    Two(Step, Step)
}

fn main() {
    let input = include_str!("./sample2.txt");
    let output = do_the_job(input);
    dbg!(output);
}

fn move_path(c: Coord, d: Direction, size: Coord) -> Option<Coord> {
    match d {
        Direction::North => if c.y > 0 { return Some(Coord{x: c.x, y: c.y - 1}) },
        Direction::South => if c.y < size.y - 1 { return Some(Coord{x: c.x, y: c.y + 1}) },
        Direction::West => if c.x > 0 { return Some(Coord{x: c.x - 1 , y: c.y }) },
        Direction::East => if c.x < size.x - 1 { return Some(Coord{x: c.x +1 , y: c.y}) },
    }

    return None;
}


fn parse_input(input: &str) -> (Coord, Coord, HashMap<Coord, NextSteps>) {
    let mut lines = input.lines().peekable();
    //let mut nodes : HashMap<Coord, Node> = HashMap::new();
    let mut nodes : HashMap<Coord,NextSteps> = HashMap::new();

    let mut start = Coord{x: 0, y: 0};
    let size = Coord{y: lines.clone().count(), x: lines.peek().unwrap().len()};

    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = Coord{x,y};
            if c == START {
                println!("FOUND START NODE AT: {:?}", coord);
                start = coord;
            } else if let Some(dir) = tag_to_pipe(c) {
                println!("{:?} '{}' => {:?}", coord, c, dir);
                let path1 = dir.0;
                if let Some(next1) = move_path(coord, path1, size) {
                    println!("\tMoving {:?} => {:?}", path1, next1); 
                    let next_step = Step{dir: path1.reverse(), to: coord};
                    let found = nodes.get(&next1);
                    match found {
                        None => {
                            println!("\t has one step: {:?}", next_step);
                            nodes.insert(next1, NextSteps::One(next_step));
                        }
                        Some(NextSteps::One(n1)) => {
                            println!("\t has two steps: {:?}, {:?}", n1, &next_step);
                            nodes.insert(next1, NextSteps::Two(*n1, next_step));
                        }
                        Some(NextSteps::Two(_,_)) => {
                            println!("\t already had two steps!");
                        },
                        _ => { unreachable!("Should not be there! {:?} next1={:?}", found, next1); } 

                    }
                }
                let path2 = dir.1;
                if let Some(next2) = move_path(coord, path2, size) {
                    println!("\tMoving {:?} => {:?}", path2, next2); 
                    let next_step = Step{dir: path2.reverse(), to: coord};
                    let found = nodes.get(&next2);
                    match found {
                        None => {
                            println!("\t has one step: {:?}", next_step);
                            nodes.insert(next2, NextSteps::One(next_step));
                        }
                        Some(NextSteps::One(n2)) => {
                            println!("\t has two steps: {:?}, {:?}", n2, &next_step);
                            nodes.insert(next2, NextSteps::Two(*n2, next_step));
                        }
                        Some(NextSteps::Two(_,_)) => (),
                        _ => { unreachable!("Should not be there!"); } 

                    }
                }
            }
        }
    }

    // reverse directions of map 

    (size, start, nodes)
}

fn do_the_job(input: &str) -> u32 {
    let (size, start, nodes) = parse_input(&input);
    println!("Map size: y={} x={}", size.y, size.x);
    println!("Map start position: y={} x={}", start.y, start.x);

    println!("NODES: {:?}", &nodes);

    let mut distance : u32 = 1;

    let start_node = nodes.get(&start).unwrap();
    let (first_path, second_path) = match start_node {
        NextSteps::Two(first_path,second_path) => {
            (first_path, second_path)
        }
        _ => { unreachable!("Start should have 2 paths coming from it"); }
    };

    let mut directions = (first_path.dir.clone(), second_path.dir.clone());
    println!("starting from {:?}, directions={:?})", start, directions);
    let mut positions = (first_path.to.clone(), second_path.to.clone());
    println!("INITIAL MOVES:\n\t{:?}\n\t{:?}", &first_path, &second_path);
    let final_state : (Coord, Coord) = (start.clone(), start.clone());

    while positions != final_state {
        println!("#### STEP: distance={}",distance);
        let left = &positions.0;
        let left_dir = &directions.0;
        if *left != start {
            let left_node = nodes.get(&left).unwrap();
            let (first_path, second_path) = match left_node {
                NextSteps::Two(first_path,second_path) => {
                    (first_path, second_path)
                }
                _ => { unreachable!("Node should have 2 paths coming from it {:?} -> {:?}", left, left_node); }
            };
            if first_path.dir == left_dir.reverse() {
                    directions.0 = second_path.dir.clone();
                    positions.0 = second_path.to.clone();
            } else {
                    directions.0 = first_path.dir.clone();
                    positions.0 = first_path.to.clone();
            }

            println!("\t[left] Moving in direction {:?} to {:?}", directions.0, positions.0); 

        }
        let right = &positions.1;
        let right_dir = &directions.1;
        if *right != start {
            let right_node = nodes.get(&right).unwrap();
            let (first_path, second_path) = match right_node {
                NextSteps::Two(first_path,second_path) => {
                    (first_path, second_path)
                }
                _ => { unreachable!("Node should have 2 paths coming from it {:?} -> {:?}", right, right_node); }
            };
            if first_path.dir == right_dir.reverse() {
                    directions.0 = second_path.dir.clone();
                    positions.0 = second_path.to.clone();
            } else {
                    directions.0 = first_path.dir.clone();
                    positions.0 = first_path.to.clone();
            }

            println!("\t[right] Moving in direction {:?} to {:?}", directions.0, positions.0); 

        }

        distance += 1;
    }

    distance
}


#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let input = ".....\n.S-7.\n.|.|.\n.L-J.\n.....";
        let result = do_the_job(input);
        assert_eq!(result, 4);
    }

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
