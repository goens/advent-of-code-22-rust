use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let file = match File::open("inputs/day1.txt"){
        Err(msg) => panic!("couldn't open file; {}", msg),
        Ok(f) => f,
    };
    let reader = BufReader::new(file);

    let mut elves = Vec::new();
    let mut elf = Vec::new();
    for (_index, errLine) in reader.lines().enumerate() {
        if let Ok(line) = errLine{
            if line.is_empty(){
                elves.push(elf);
                elf = Vec::new();
            }
            else{
              let i = match line.parse::<i32>(){
                  Ok(i) => i,
                  Err(e) => panic!("couldn't parse int; {}", e),
              };
              elf.push(i);
            }
        }
    }
    let mut total_cals = Vec::new();
    for elf in elves{
        let mut cals = 0;
        for c in elf{
            cals += c;
        }
        total_cals.push(cals);
    }
    total_cals.sort();
    let max_cals = total_cals.pop().unwrap();
    let three_max = max_cals + total_cals.pop().unwrap() + total_cals.pop().unwrap();
    println!("max cals: {}", max_cals);
    println!("top 3 max cals: {}", three_max);
}
