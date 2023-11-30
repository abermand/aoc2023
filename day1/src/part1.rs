fn main() {
    let input = include_str!("./sample1.txt");
    let output = do_the_job(input);
    dbg!(output);
}

fn do_the_job(input: &str) ->String {
    "ok".to_string()
}


#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = do_the_job("Hello");
        assert_eq!(result, "ok");
    }
}