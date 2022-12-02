use std::io::{BufReader, BufRead};
use std::fs::File;
mod day1;
mod day2;

macro_rules! run {
	  ($x:ident) => {
        println!("\n\nRunning {}:", stringify!($x));
        let file = match File::open(format!("inputs/{}.txt", stringify!($x))){
          Err(msg) => panic!("couldn't open file; {}", msg),
          Ok(f) => f,
      };
      let reader = BufReader::new(file);
      $x::$x(reader.lines());
	  };
}

fn main() {
    run!(day1);
    run!(day2);
}
