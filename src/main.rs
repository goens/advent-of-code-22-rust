use std::io::{BufReader, BufRead};
use std::fs::File;
mod day1;
mod day2;

fn main() {
    println!("Day 1:");
    let file = match File::open("inputs/day1.txt"){
        Err(msg) => panic!("couldn't open file; {}", msg),
        Ok(f) => f,
    };
    let reader = BufReader::new(file);
    day1::run(reader.lines());

    println!("\n\nDay 2:");
    // TODO: can I use a macro to do this?
    let file = match File::open("inputs/day2.txt"){
        Err(msg) => panic!("couldn't open file; {}", msg),
        Ok(f) => f,
    };
    let reader = BufReader::new(file);
    day2::run(reader.lines());

}
