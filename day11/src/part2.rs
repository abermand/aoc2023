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

use std::{cmp, fmt::Display};
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
    x: usize, 
    y: usize,
}

#[derive(Debug, Clone)]
struct Universe {
    width: usize,
    height: usize,
    contents: Vec<Vec<Option<Galaxy>>>,
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = "".to_string();
        for row in self.contents.iter() {
            for e in row.iter() {
                s.push_str(format!("{}", match e { Some(_) => "#", None => "."}).as_str());
            }
            for e in row.iter() {
                if let Some(g) = e {
                    s.push_str(format!(" #{}({},{})", g.id, g.x, g.y).as_str());
                }
            }
            s.push_str("\n");
        }
        write!(f,"{}\n", s)
    }
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

    fn distance(g1: &Galaxy, g2: &Galaxy) -> u64 {
        let d = ((g2.x as i32 - g1.x as i32).abs() as u64) + ((g2.y as i32 - g1.y as i32).abs() as u64);
        // println!("D: ({},{}) -> ({},{}) = {}", g1.x, g1.y, g2.x, g2.y, d);
        d
    }

    fn expand(self: &mut Self, factor: usize) -> () {
        
        // first find empty columns: 
        let empty_columns : Vec<usize> = (0..self.width).filter(|x| {
            return (0..self.height).all(|y| self.contents[y][*x] == None);
        }).collect();
        // println!("Empty columns: {:?}", &empty_columns);

        let mut y_stack = 0;
        // then find empty rows to expand vertically
        for y in 0..self.height {
            let row: &mut Vec<Option<Galaxy>> = self.contents[y].as_mut();
            if row.iter().all(|x| *x == None) {
                // println!("found an empty row #{}", y);
                y_stack += factor-1;
            }
            for mut e in row.iter_mut() {
                if let Some(g) = &mut e {
                    g.y += y_stack;
                }                
            }
        }
        // println!("After expanding vertically: {:?}", &self.contents);
        //then expand horizontally: 
        let mut x_stack = 0;
        for x in 0 .. self.width {
            if empty_columns.contains(&x) {                
                // println!("Column {} is empty", &x);
                x_stack += factor-1;
            }
            // println!("Expanding horizontally for col #{} (stack={})", x, x_stack);
            for y in 0..self.height {
                let row: &mut Vec<Option<Galaxy>> = self.contents[y].as_mut();
                let mut e = row[x].as_mut();
                if let Some(g) = &mut e {
                    // println!("Increased: {} -> {}", &g.x, &g.x + x_stack);
                    g.x = g.x + x_stack;
                } 
            }
        }
    }
}

fn main() {
    let input = include_str!("./sample2.txt");
    let output = do_the_job(input, 1_000_000);
    dbg!(output);
}


fn parse_input(input: &str) -> Universe {
    let mut height: usize  = 0;
    let mut id = 0;
    let contents : Vec<Vec<Option<Galaxy>>> = input.lines().enumerate().map(|(y, line)| {
        height += 1;
        line.chars().enumerate().map(move |(x, c)| {
            match c {
                '#' => { id += 1; Some(Galaxy{id, x, y}) }
                _ => None,
            }
        }).collect::<Vec<Option<Galaxy>>>()
    }).collect();
    let width = contents[0].len();
    Universe{width, height, contents}
}


fn do_the_job(input: &str, factor: usize) -> u64 {
    let mut u = parse_input(&input);
    println!("BEFORE EXPAND:\n{}", &u);
    u.expand(factor);
    println!("AFTER EXPAND:\n{}", &u);
    let galaxies: Vec<Galaxy> = u.contents.iter()
        .flatten().filter_map(|e| {
            match e {
                Some(_) => *e,
                None => None,
            }
        }).collect();
        let len = galaxies.len();
        let mut result : u64 = 0;
        for i1 in 0..len {
            for i2 in i1+1..len {
                result += Universe::distance(&galaxies[i1], &galaxies[i2]);
            }
        }
    result
}


#[cfg(test)] 
mod tests {
    use super::*;

    #[rstest]
    #[case(2, 374)]
    #[case(10,1030)]
    #[case(100,8410)]
    fn test_expand(#[case] factor: usize, #[case] expected: u64) {
        let input = include_str!("./sample1.txt");
        let result = do_the_job(input, factor);
        assert_eq!(result, expected);
    }
}