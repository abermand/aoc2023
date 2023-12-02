use std::cmp;
use rstest::*;

fn main() {
    let input = include_str!("./sample2.txt");
    let output = do_the_job(input);
    dbg!(output);
}

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
    fn power(self: &Game) -> i32 {
        self.red * self.green * self.blue
    }
}

fn do_the_job(input: &str) -> i32 {
   let lines = input.lines();

    let mut result = 0i32;
    
    for line in lines {
        let game = Game::parse_line(line);
        result += game.power();
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
        assert_eq!(result, 2286);
    }

    #[rstest]
    #[case("Game 95: 1 blue, 11 red; 15 red, 1 blue, 3 green; 13 red, 2 blue, 3 green; 1 green, 1 blue", 15 * 3 * 2)]
#[case("Game 98: 8 red, 12 green, 2 blue; 7 green, 8 red, 1 blue; 2 blue, 6 red, 3 green; 9 red, 1 blue, 4 green",9 * 12 * 2)]
    #[case("Game 5: 17 red, 5 blue, 3 green; 8 green, 9 red, 10 blue; 2 green, 9 blue, 4 red", 17 * 8 * 10)]
    fn is_game_power_ok(#[case] line: &str, #[case] expected: i32) {
        let game = Game::parse_line(line);
        assert_eq!(game.power(), expected);
    }
}
