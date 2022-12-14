use std::io::{BufReader, Lines};
use std::fs::File;


pub fn run(lines : Lines<BufReader<File>>) {
    let mut elves = Vec::new();
    let mut elf = Vec::new();
    for (_index, err_line) in lines.enumerate() {
        if let Ok(line) = err_line{
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
