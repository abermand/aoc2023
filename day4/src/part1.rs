use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::collections::HashSet;
//use rstest::*;

lazy_static! {
    static ref NUMBERS_REGEX : Regex = Regex::new(r"(\d+)").unwrap();
    //static ref SYMBOLS_REGEX : Regex = Regex::new(r"[^0-9.]").unwrap();
}

struct Game {
    id: u32,
    winners: Vec<u32>,
    draws: Vec<u32>,
}

impl Game {
    fn parse(line: &str) -> Game {
        let (header, rest) = line.split_once(": ").unwrap();
        println!("Line: {}",&line);
        let id : u32 = header.split_once(" ").unwrap().1.trim().parse::<u32>().expect("id should be an int");

        let (winners, draws) = rest.trim().split_once(" | ").unwrap();
        let winners : Vec<u32> = NUMBERS_REGEX.find_iter(winners)
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .collect();
        let draws : Vec<u32> = NUMBERS_REGEX.find_iter(draws)
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .collect();

        Game {
            id,
            winners,
            draws,
        }
    }

    fn score(self: &Game) -> u32 {
       let w : HashSet<&u32> = self.winners.iter().collect::<HashSet<_>>();
       let d : HashSet<&u32> = self.draws.iter().collect::<HashSet<_>>();

        let i : Vec<&u32> = w.intersection(&d).map(|x| *x).collect();
        let len = i.len();
        let score = if len > 0 { 2u32.pow((i.len() as u32) - 1) } else { 0 };
        println!("Game {} matches: {:?} score={}", self.id, i, score);
        score
    }

}

fn main() {
    let input = include_str!("./sample2.txt");
    let output = do_the_job(input);
    dbg!(output);
}

fn do_the_job(input: &str) -> u32 {
   let games = input.lines()
        .map(Game::parse);

    let result : u32 = games.map(|g| g.score()).sum::<u32>();

    result
}


#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./sample1.txt");
        let result = do_the_job(input);
        assert_eq!(result, 13);
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
