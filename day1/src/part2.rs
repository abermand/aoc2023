// used to find overlapping matches, because regex::Regex does not support overlapping or lookahead
use aho_corasick::AhoCorasick;

fn main() {
    let input = include_str!("./sample1.txt");
    let output = do_the_job(input);
    dbg!(output);
}

static PATTERNS : [&str; 18] = [
    "one", "1",
    "two", "2",
    "three", "3", 
    "four", "4",
    "five", "5",
    "six", "6", 
    "seven", "7", 
    "eight", "8",
    "nine", "9",
];


fn string_to_number(s: &str) -> i32 {
    match s {
    "one" => 1,
    "two" => 2,
    "three" => 3, 
    "four" => 4,
    "five" => 5,
    "six" => 6, 
    "seven" => 7, 
    "eight" => 8,
    "nine" => 9,
     x => x.parse::<i32>().unwrap(),
    }
}

fn do_the_job(input: &str) -> i32 {
   let lines = input.lines();
   let mut result : i32 = 0;
   let ac = AhoCorasick::new(PATTERNS).unwrap();
    for (_i, line) in lines.enumerate() {
        let numbers : Vec<_> = ac
            .find_overlapping_iter(line)
            .map(|m| {
                let s = &line[m.start()..m.end()];
                string_to_number(s)
            })
            .collect();
        let digits : i32 = match numbers.len() {
            0 => 0,
            _ => 10 * numbers.first().unwrap() + numbers.last().unwrap(),
        };
        // println!("Line #{} {} -> {:?} -> {}", i+1, &line, &numbers, &digits);
        result += digits;
    }
    result
}


#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn it_works_simple() {
        let input = "two1nine";
        let result = do_the_job(input);
        assert_eq!(result, 29);
        let input = "eighthree";
        let result = do_the_job(input);
        assert_eq!(result, 83);
        let input = "sevenine";
        let result = do_the_job(input);
        assert_eq!(result, 79);
        
    }
    
    #[test]
    fn it_works_2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneeight234
7pqrstsixteen";
        let result = do_the_job(input);
        assert_eq!(result, 281);
    }

    #[test]
    fn it_works_last_10() {
        let input = "nfzlonesmeight6gtff
tleighttdxtbhrvgk16bpkmtcvlnrhnmhz
one99
mqtwooneeight7sevenfourht
stzmqplr8gvmxblz
five8dvdjqfmpnh3
scjjr4twoh
nine35gzmlt
5sixfour2qxsqkpnq
kdkjqdkvgs2";

        let result = do_the_job(input);
        assert_eq!(result, 497);

    }
}
