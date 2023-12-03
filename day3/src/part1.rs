use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
//use rstest::*;

lazy_static! {
    static ref NUMBERS_REGEX : Regex = Regex::new(r"(\d+)").unwrap();
    static ref SYMBOLS_REGEX : Regex = Regex::new(r"[^0-9.]").unwrap();
}


fn main() {
    let input = include_str!("./sample2.txt");
    let output = do_the_job(input);
    dbg!(output);
}

fn truncated_slice(s: &str, start: usize, end: usize) -> &str {
    let ts : usize = start.saturating_sub(1);
    let te : usize = cmp::min(s.len(), end+1);
    &s[ts..te]
}

fn get_parts_sum(current: &str, previous: Option<&str>, next: Option<&str>) -> u32 {
    let mut result : Vec<u32> = Vec::new();
    let len = current.len();
    // find number matches in line:
    for m in NUMBERS_REGEX.find_iter(current) {
        let start = m.start();
        let end = m.end();
        let mut is_part = false;
        let val = u32::from_str_radix(&current[start..end],10).unwrap();
        // look at char on the left
        if start > 0 && SYMBOLS_REGEX.is_match(&current[start-1..start]) {
            is_part = true;
        // look at char on the right
        } else if end+1 < len && SYMBOLS_REGEX.is_match(&current[end..end+1]) {
            is_part = true;
        // look at the previous row
        } else {
            if let Some(p) = previous {
                if SYMBOLS_REGEX.is_match(truncated_slice(p, start, end)) {
                    is_part = true;
                }
            }
            if let Some(n) = next {
                if SYMBOLS_REGEX.is_match(truncated_slice(n, start, end)) {
                    is_part = true;
                }
            }
        }
        if is_part {
            println!("Line '{}' has parts: {:?}", current, &result);
            result.push(val);
        } else {
            println!("Line '{}' has no parts", current);
        }
    }
    return result.iter().sum();
}

fn do_the_job(input: &str) -> u32 {
   let lines : Vec<&str> = input.lines().collect();

    let mut result = 0u32;
    let len = lines.len();
    
    for index in 0..len {
        let previous = if index > 0 { lines.get(index-1) } else { None };
        let current = lines.get(index).unwrap();
        let next = lines.get(index+1);
        result += get_parts_sum(current,previous.copied(),next.copied());
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
        assert_eq!(result, 4361);
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
