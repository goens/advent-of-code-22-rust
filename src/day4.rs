use std::io::{BufReader, Lines};
use std::fs::File;


pub fn run(lines : Lines<BufReader<File>>) {

    let mut repeated_pairs = 0;
    let mut overlapping_pairs = 0;
    for (_index, err_line) in lines.enumerate() {
        let line = if let Ok(line) = err_line {line} else { panic!() };
        let (fst,snd) = if let Some((fst,snd)) = line.split_once(',') {(fst,snd)} else { panic!() };
        let (fst_lhs_str,fst_rhs_str) = if let Some((fst_lhs_str,fst_rhs_str)) = fst.split_once('-') {(fst_lhs_str, fst_rhs_str)} else { panic!() };
        let (snd_lhs_str,snd_rhs_str) = if let Some((snd_lhs_str,snd_rhs_str)) = snd.split_once('-') {(snd_lhs_str, snd_rhs_str)} else { panic!() };
        let fst_lhs = if let Ok(fst_lhs) = fst_lhs_str.parse::<u32>() {fst_lhs} else { panic!() };
        let snd_lhs = if let Ok(snd_lhs) = snd_lhs_str.parse::<u32>() {snd_lhs} else { panic!() };
        let fst_rhs = if let Ok(fst_rhs) = fst_rhs_str.parse::<u32>() {fst_rhs} else { panic!() };
        let snd_rhs = if let Ok(snd_rhs) = snd_rhs_str.parse::<u32>() {snd_rhs} else { panic!() };
        // let Ok(line) = err_line else { panic!() };
        // let Some((fst,snd)) = line.split_once(',') else { panic!() };
        // let Some((fst_lhs_str,fst_rhs_str)) = fst.split_once('-') else { panic!() };
        // let Some((snd_lhs_str,snd_rhs_str)) = fst.split_once('-') else { panic!() };
        // let Ok(fst_lhs) = fst_lhs_str.parse::<u32>() else { panic!() };
        // let Ok(snd_lhs) = snd_lhs_str.parse::<u32>() else { panic!() };
        // let Ok(fst_rhs) = fst_rhs_str.parse::<u32>() else { panic!() };
        // let Ok(snd_rhs) = snd_rhs_str.parse::<u32>() else { panic!() };
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
