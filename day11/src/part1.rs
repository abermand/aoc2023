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
use rstest::*;

lazy_static! {
    static ref NUMBERS_REGEX : Regex = Regex::new(r"(\d+)").unwrap();
    //static ref SYMBOLS_REGEX : Regex = Regex::new(r"[^0-9.]").unwrap();
    //static ref MAP_REGEX: Regex = Regex::new(r"(\S+)-to-(\S+) map:").unwrap();
}

#[derive(Debug,PartialEq, Eq, Clone, Copy)]
struct Galaxy {
    id: u32,
}

#[derive(Debug, Clone)]
struct Universe {
    width: usize,
    height: usize,
    contents: Vec<Vec<Option<Galaxy>>>,
}

impl Universe {
    fn get(self: &Self, x: usize, y: usize) -> Option<Galaxy> {
        self.contents.get(y).and_then(|row| row.get(x)).copied().flatten()
    }
    fn coords(self: &Self, i: usize) -> (usize, usize) { 
        let x = i % self.width;
        let y = (i-x) / self.width;
        (x,y)
    }

    fn distance(g1: (usize, usize), g2: (usize, usize)) -> u32 {
        ((g2.0 as i32 - g1.0 as i32).abs() as u32) + ((g2.1 as i32 - g1.1 as i32).abs() as u32)
    }

    fn expand(self: &Self) -> Self {
        let mut new_height = self.height;
        let mut new_width = self.width;
        let mut new_contents : Vec<Vec<Option<Galaxy>>> = vec![];
        
        // first find empty columns: 
        let empty_columns : Vec<usize> = (0..self.width).filter(|x| {
            return (0..self.height).all(|y| self.contents[y][*x] == None);
        }).collect();
        println!("Empty columns: {:?}", &empty_columns);
        new_width += empty_columns.len();


        // then find empty rows to expand vertically
        for y in 0..self.height {
            let row = &self.contents[y];
            if row.iter().all(|x| *x == None) {
                println!("found an empty row #{}", y);
                // insert a blank row
                new_contents.push([None].repeat(self.width));
                new_height += 1;
            }
            new_contents.push(row.to_vec());
        }
        println!("After expanding vertically: {:?}", &new_contents);
        //then expand horizontally (from right to left to not disturb order): 
        for x in empty_columns.iter().rev() {
            println!("Expanding horizontally for col #{}", x);
            for y in 0..new_height {
                new_contents[y].insert(*x, None);
            }
        }
        
        Self{width: new_width, height: new_height, contents: new_contents}
    }
}

fn main() {
    let input = include_str!("./sample2.txt");
    let output = do_the_job(input);
    dbg!(output);
}


fn parse_input(input: &str) -> Universe {
    let mut height: usize  = 0;
    let mut id = 0;
    let contents : Vec<Vec<Option<Galaxy>>> = input.lines().enumerate().map(|(y, line)| {
        height += 1;
        line.chars().enumerate().map(move |(x, c)| {
            match c {
                '#' => { id += 1; Some(Galaxy{id}) }
                _ => None,
            }
        }).collect::<Vec<Option<Galaxy>>>()
    }).collect();
    let width = contents[0].len();
    Universe{width, height, contents}
}


fn do_the_job(input: &str) -> u32 {
    let u = parse_input(&input);
    let u = u.expand();
    let galaxies: Vec<(usize, usize)> = u.contents.iter()
        .flatten().enumerate().filter_map(|(i, e)| {
            match e {
                Some(_) => Some(u.coords(i)),
                None => None,
            }
        }).collect();
        let len = galaxies.len();
        let mut result : u32 = 0;
        for i1 in 0..len {
            for i2 in i1+1..len {
                result += Universe::distance(galaxies[i1], galaxies[i2]);
            }
        }
    result
}


#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./sample1.txt");
        let result = do_the_job(input);
        assert_eq!(result, 374);
    }

    #[rstest]
    #[case("#","#")]
    #[case(".","..\n..")]
    #[case("..\n..","....\n....\n....\n....")]
    #[case("..\n#.","...\n...\n#..")]
    // ..  -> ...
    // #.  -> ...
    //     -> #..
    fn test_expand(#[case] line1: &str, #[case] line2: &str) {
        let u1 = parse_input(&line1);
        println!("INITIAL: {:?}", &u1);
        let u2 = parse_input(&line2);
        let eu1 = u1.expand();
        println!("EXPANDED: {:?}", &eu1);
        println!("EXPECTED: {:?}", &u2);
        assert_eq!(eu1.width, u2.width);
        assert_eq!(eu1.height, u2.height);
        assert_eq!(eu1.contents, u2.contents);

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
