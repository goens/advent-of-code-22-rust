use std::io::{BufReader, BufRead};
use std::fs::File;
mod day1;

fn main() {
    let file = match File::open("inputs/day1.txt"){
        Err(msg) => panic!("couldn't open file; {}", msg),
        Ok(f) => f,
    };
    let reader = BufReader::new(file);
    day1::run(reader.lines());
}
