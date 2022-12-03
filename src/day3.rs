use std::io::{BufReader, Lines};
use std::fs::File;

fn to_priorities(obj : char) -> u32{
    // something that's encoding independent and not a huge match?
    let ascii_val = obj as u32;
    if obj.is_uppercase() {
        ascii_val - ('A' as u32) + 27
    } else {
        assert!(obj.is_lowercase());
        ascii_val - ('a' as u32) + 1
    }
}

pub fn run(lines : Lines<BufReader<File>>) {
    let mut fst = String::new();
    let mut snd = String::new();
    let mut total = 0;
    let mut total_groups = 0;
    for (index, err_line) in lines.enumerate() {
        if let Ok(line) = err_line{
            //part I
            if line.len() % 2 != 0 {
                println!("line {index} is uneven({})! Line: {line}", line.len());
            }
            let mut lhs = line.clone();
            let rhs = lhs.split_off(lhs.len() / 2);
            let mut joint = None;
            assert!(lhs.len() == rhs.len());
            for x in rhs.chars(){
                for y in lhs.chars(){
                    if x == y{
                        if joint.is_some() && joint.unwrap() != x {
                            println!("two joint chars in line {index}! {}, {x}", joint.unwrap());
                        }
                        joint = Some(x);
                    }
                }
            }
            if let Some(val) = joint{
              total += to_priorities(val);
            }
            // part II
            match index % 3{
                0 => fst = line,
                1 => snd = line,
                2 => {
                    let mut first_two_joint = Vec::new();
                    for x in fst.chars(){
                        for y in snd.chars(){
                            if x == y{
                                first_two_joint.push(x);
                            }
                        }
                    }
                    let mut found = false;
                    for x in first_two_joint{
                        for z in line.chars(){
                            if x == z {
                                total_groups += to_priorities(z);
                                found = true;
                                break;
                            }
                        }
                        if found {
                            break;
                        }
                    }

                }
                _ => unreachable!()
            }


        }
    }
    println!("total: {total}");
    println!("total groups: {total_groups}");
}
