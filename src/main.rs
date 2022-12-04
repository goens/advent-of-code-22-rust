use std::io::{BufReader, BufRead};
use std::fs::File;
mod day1;
mod day2;
mod day3;
mod day4;

macro_rules! run {
	  ($x:ident) => {
        println!("\n\nRunning {}:", stringify!($x));
        let file = match File::open(format!("inputs/{}.txt", stringify!($x))){
          Err(msg) => panic!("couldn't open file; {}", msg),
          Ok(f) => f,
      };
      let reader = BufReader::new(file);
      $x::run(reader.lines());
	  };
}

fn main() {
    run!(day1);
    run!(day2);
    run!(day3);
    run!(day4);
}
