use std::io::{BufReader, Lines};
use std::fs::File;


pub fn run(lines : Lines<BufReader<File>>) {

    let mut repeated_pairs = 0;
    let mut overlapping_pairs = 0;
    for (_index, err_line) in lines.enumerate() {
        let Ok(line) = err_line else { panic!() };
        let Some((fst,snd)) = line.split_once(',') else { panic!() };
        let Some((fst_lhs_str,fst_rhs_str)) = fst.split_once('-') else { panic!() };
        let Some((snd_lhs_str,snd_rhs_str)) = snd.split_once('-') else { panic!() };
        let Ok(fst_lhs) = fst_lhs_str.parse::<u32>() else { panic!() };
        let Ok(snd_lhs) = snd_lhs_str.parse::<u32>() else { panic!() };
        let Ok(fst_rhs) = fst_rhs_str.parse::<u32>() else { panic!() };
        let Ok(snd_rhs) = snd_rhs_str.parse::<u32>() else { panic!() };
        if fst_lhs <= snd_lhs && fst_rhs >= snd_rhs{ // fst \supseteq snd
            repeated_pairs += 1;
        } else if fst_lhs >= snd_lhs && fst_rhs <= snd_rhs{
            repeated_pairs += 1;
        }
        if fst_rhs >= snd_lhs && fst_lhs <= snd_lhs {
            overlapping_pairs += 1;
        } else if snd_rhs >= fst_lhs && snd_lhs <= fst_lhs {
            overlapping_pairs += 1;
        }
    }
    println!("pairs included in the other : {repeated_pairs}");
    println!("pairs with some overlap : {overlapping_pairs}");
}
