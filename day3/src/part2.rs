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

struct Part {
    val : u32,
    //row : usize,
    start : usize,
    end : usize,
}

impl Part {
    fn len(self: &Part) -> usize {
        self.end - self.start
    }
}


fn get_parts_and_stars(current: &str, previous: Option<&str>, next: Option<&str>) -> (Vec<Part>, Vec<usize>) {
    let mut parts : Vec<Part> = Vec::new();
    let mut stars : Vec<usize> = Vec::new();
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
            // println!("Line '{}' has parts: {:?}", current, &result);
            parts.push(Part{ start, end, val });
        } else {
            println!("Line '{}' has no parts", current);
        }
    }
    for (pos,c) in current.char_indices() {
        if c == '*' {
            stars.push(pos);
        }
    }
    return (parts, stars);
}

fn is_adjacent(star:usize, start: usize, end: usize, len: usize) -> bool {
    //add +1 on each side of [start..end] range:
    let ts : usize = start.saturating_sub(1);
    let te : usize = cmp::min(len, end);
    println!("\t\tis_adjacent({}) in [{}:{}] -> [{}:{}]? {}", 
        star, start, end, ts, te,
        (star >= ts) && (star <= te));
    return star >= ts && star <= te
}


fn get_adjacent_parts(
    star: usize, 
    previous : Option<&Vec<Part>>,
    current : &Vec<Part>,
    next: Option<&Vec<Part>>,
    len: usize) -> Vec<u32> {
    let mut adj : Vec<u32> = vec![];
    for p in current {
        if is_adjacent(star, p.start, p.end, len) {
            adj.push(p.val);
        }
    }
    if let Some(previous) = previous {
        for p in previous {
            if is_adjacent(star, p.start, p.end, len) {
                adj.push(p.val);
            }
        }
    }
    if let Some(next) = next {
        for p in next {
            if is_adjacent(star, p.start, p.end, len) {
                adj.push(p.val);
            }
        }
    }
    
    adj
}

fn do_the_job(input: &str) -> u32 {
   let lines : Vec<&str> = input.lines().collect();

    let mut result = 0u32;
    let mut parts : Vec<Vec<Part>> = vec![vec![]];
    let mut stars : Vec<Vec<usize>> = vec![vec![]];
    let len = lines.len();
    let mut line_len = 0;
    
    // first parse the string into parts and stars
    for index in 0..len {
        let previous = if index > 0 { lines.get(index-1) } else { None };
        let current = lines.get(index).unwrap();
        if line_len == 0 { line_len = current.len(); }
        let next = lines.get(index+1);
        let (p, s) = get_parts_and_stars(current,previous.copied(),next.copied());
        parts.push(p);
        stars.push(s);
    }

    // then determine if 'stars' are adjacent to 2 parts:
    for index in 0..len {
        let current_stars = stars.get(index).unwrap();
        if current_stars.is_empty() {
            continue;
        }
        println!("--- Line #{} current_stars {:?}", index, &current_stars);
        let previous = if index > 0 { parts.get(index-1) } else { None };
        let current = parts.get(index).unwrap();
        let next = parts.get(index+1);

        for star in current_stars {
            let adjacent_parts = get_adjacent_parts(*star, previous, current, next, line_len);
            println!("Looking at star pos {} on line {}: adjacent: {:?}", star, &index, &adjacent_parts);
            if adjacent_parts.len() == 2 {
                let ratio = adjacent_parts.iter().product::<u32>();
                println!("\tstar pos {} on line {} has 2 adjacent parts: RATIO={}", star, index, &ratio);
                result += ratio;
            }
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
        assert_eq!(result, 467835);
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
