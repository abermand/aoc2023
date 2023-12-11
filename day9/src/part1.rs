use lazy_static::lazy_static;
use regex::Regex;
use rstest::*;

lazy_static! {
    static ref NUMBERS_REGEX : Regex = Regex::new(r"(\d+)").unwrap();
    //static ref SYMBOLS_REGEX : Regex = Regex::new(r"[^0-9.]").unwrap();
    //static ref MAP_REGEX: Regex = Regex::new(r"(\S+)-to-(\S+) map:").unwrap();
}

#[derive(Debug)]
struct Struct {
    }

fn main() {
    let input = include_str!("./sample2.txt");
    let output = do_the_job(input);
    dbg!(output);
}

fn recurse_sum(input: &mut Vec<i64>, index: usize) -> i64 {
    println!("Recurse #{} {:?}", index, &input);
    if input[index..].iter().all(|x| *x == 0i64) { 
        println!("\tRecursion ends at index {}", index);
        return 0; 
    }
    let mut next_vec = input.to_vec();
    if index+1  < input.len() {
        for i in index .. input.len()-1  {
            // println!("\t\t #{} Substracting {} to {}", i, &input[i], &next_vec[i+1]);
            next_vec[i+1] -= input[i];
        }
    }
    println!("\there's our array after subtracting: {:?}", &next_vec[index..]);
    next_vec[index] = 0;
    let sum = recurse_sum(&mut next_vec, index+1);
    let last_index = input[index..].len() -1;
    println!("Adding {} to element {} of {:?} ({})",sum, last_index, &input[index..], input[index+last_index]);
    //println!("\t output {:?} -> {}", &input, &sum + input[index] );
    return sum+input[index+last_index];
}


fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input.lines().map(|line| {
        line.split(" ").filter_map(|x| if let Ok(n) = x.parse::<i64>() { Some(n) } else { None }).collect::<Vec<i64>>()
    }).collect()
}

fn do_the_job(input: &str) -> i64 {
    let inputs = parse_input(&input);
    let mut result = 0i64;
    for mut input in inputs {
        let sum = recurse_sum(&mut input, 0);
        println!("Result for the line: {}", sum);
        result += sum;
    }
    result
}


#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn it_works_single() {
        let input = "0 3 6 9 12 15";
        let result = do_the_job(input);
        assert_eq!(result, 18);
    }
    #[test]
    fn it_works() {
        let input = include_str!("./sample1.txt");
        let result = do_the_job(input);
        assert_eq!(result, 114);
    }

    #[rstest]
    #[case("0 3 6 9 12 15", 18)]
    #[case("1   3   6  10  15  21", 28)]
    #[case("10  13  16  21  30  45", 68)]
    fn test_single_line(#[case] line : &str ,#[case] expected : i64) {
        let result = do_the_job(line);
        assert_eq!(result, expected);
    }
}
