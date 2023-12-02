//use lazy_static::lazy_static;
//use regex::Regex;
use std::cmp;
use rstest::*;

fn main() {
    let input = include_str!("./sample2.txt");
    let output = do_the_job(input);
    dbg!(output);
}

//#[derive(Debug)]
//enum Colours {
 //   Red, 
  //  Blue,
   // Green,
//}

struct Game {
    pub id: i32,
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

impl Game {
    fn parse_line(line: &str) -> Game {
        let (header, game) = line.split_once(": ").unwrap();

        let (_, id) = header.split_once(" ").unwrap();
        let id : i32 = id.parse().unwrap();
        let mut red = 0i32;
        let mut green = 0i32;
        let mut blue = 0i32;

        for round in game.split("; ") {
            for draw in round.split(", ") {
                let (val, colour) = draw.split_once(" ").unwrap();
                // println!("draw '{}' val '{}' colour '{}'", &draw, &val, &colour);
                let val = i32::from_str_radix(val, 10).unwrap();
                if colour == "red" {
                    red = cmp::max(red, val);
                } else if colour == "green" {
                    green = cmp::max(green, val);
                } else if colour == "blue" {
                    blue = cmp::max(blue, val);
                }
            }

        }

        println!("Game {}: \"{}\" -> R{} G{} B{}", id, &line, red, green, blue); 
        return Game {
            id,
            red,
            blue,
            green,
        }; 
    }
    fn is_possible(self: &Game) -> bool {
        (self.red <= 12) && (self.green <= 13) && (self.blue <= 14)
    }
}

fn do_the_job(input: &str) -> i32 {
   let lines = input.lines();

    let mut result = 0i32;
    
    for line in lines {
        let game = Game::parse_line(line);
        if game.is_possible() {
            result += game.id;
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
        assert_eq!(result, 8);
    }

    #[test]
    fn it_works_10() {
        let input = "Game 1: 7 blue, 5 red; 10 red, 7 blue; 5 blue, 4 green, 15 red; 4 green, 6 red, 7 blue; 5 green, 8 blue, 4 red; 5 red, 4 blue, 3 green
Game 2: 8 green, 3 red; 7 blue, 6 red, 8 green; 7 blue, 3 green, 6 red; 8 green, 6 blue, 11 red; 6 blue, 3 green, 12 red
Game 3: 6 blue, 3 red, 7 green; 3 red, 3 green, 8 blue; 8 blue, 11 red, 4 green; 5 blue, 7 red, 6 green; 9 blue, 7 green, 1 red
Game 4: 3 red, 4 green; 5 red, 1 blue; 2 green; 3 green, 1 blue; 2 green, 1 blue, 1 red
Game 5: 17 red, 5 blue, 3 green; 8 green, 9 red, 10 blue; 2 green, 9 blue, 4 red
Game 6: 5 blue, 6 green, 3 red; 1 green, 8 blue, 12 red; 2 blue, 13 red, 6 green
Game 7: 1 green, 1 blue, 6 red; 1 red, 8 green; 3 red, 8 green, 2 blue; 14 green, 4 blue, 4 red; 4 green, 5 blue; 7 green, 2 blue, 1 red
Game 8: 6 blue, 9 red, 3 green; 2 red, 6 blue; 2 green, 1 red, 2 blue; 2 green, 9 blue, 6 red
Game 9: 5 green, 8 blue, 8 red; 2 blue, 6 green, 8 red; 6 red, 9 green
Game 10: 2 red, 2 blue, 12 green; 8 green, 3 red; 5 blue, 11 red, 6 green; 14 red, 1 green";
        let result = do_the_job(input);
        assert_eq!(result, 2 + 3 + 4 + 8 + 9);
    }


    #[rstest]
    #[case("Game 95: 1 blue, 11 red; 15 red, 1 blue, 3 green; 13 red, 2 blue, 3 green; 1 green, 1 blue", false)]
#[case("Game 98: 8 red, 12 green, 2 blue; 7 green, 8 red, 1 blue; 2 blue, 6 red, 3 green; 9 red, 1 blue, 4 green", true)]
    #[case("Game 5: 17 red, 5 blue, 3 green; 8 green, 9 red, 10 blue; 2 green, 9 blue, 4 red", false)]
    fn is_game_possible(#[case] line: &str, #[case] expected: bool) {
        let game = Game::parse_line(line);
        assert_eq!(game.is_possible(), expected);
    }
}
