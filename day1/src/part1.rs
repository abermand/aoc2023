use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input = include_str!("./sample1.txt");
    let output = do_the_job(input);
    dbg!(output);
}

lazy_static! {
    static ref NUMBERS_REGEX : Regex = Regex::new(r"\d").unwrap();
}

fn do_the_job(input: &str) -> i32 {
   let lines = input.lines();
   let mut result : i32 = 0;
    for line in lines {
        let numbers : Vec<_> = NUMBERS_REGEX
            .find_iter(line)
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect();
        let digits : i32 = match numbers.len() {
            0 => 0,
            _ => 10 * numbers.first().unwrap() + numbers.last().unwrap(),
        };
        result += digits;
    }
    result
}


#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "1abc2
pqr3stu8vwx";
        let result = do_the_job(input);
        assert_eq!(result, 50);
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = do_the_job(input);
        assert_eq!(result, 142);
    }
}
