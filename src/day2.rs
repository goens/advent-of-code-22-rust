use std::io::{BufReader, Lines};
use std::fs::File;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug, PartialEq)]
enum GameResult{
    You,
    Enemy,
    Draw
}

#[derive(Debug, PartialEq)]
struct Strategy {
    enemy : RockPaperScissors,
    you_naive : RockPaperScissors,
    you_actual : RockPaperScissors
}

impl FromStr for Strategy {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s.split_once(' ')
        {
            Some((enemy, you)) => {
                let enemy_fromstr = match enemy{
                    "A" => Ok(RockPaperScissors::Rock),
                    "B" => Ok(RockPaperScissors::Paper),
                    "C" => Ok(RockPaperScissors::Scissors),
                    str => Err(format!("Can't parse Rock/Paper/Scissors \"{str}\" for enemy")),
                };
                let you_naive_fromstr = match you{
                    "X" => Ok(RockPaperScissors::Rock),
                    "Y" => Ok(RockPaperScissors::Paper),
                    "Z" => Ok(RockPaperScissors::Scissors),
                    str => Err(format!("Can't parse Rock/Paper/Scissors \"{str}\" for you")),
                };
                let you_actual_fromstr = match you{
                    "X" => match enemy_fromstr{ // need to lose
                        Ok(RockPaperScissors::Rock) => Ok(RockPaperScissors::Scissors),
                        Ok(RockPaperScissors::Paper) => Ok(RockPaperScissors::Rock),
                        Ok(RockPaperScissors::Scissors) => Ok(RockPaperScissors::Paper),
                        Err(msg) => return Err(msg),
                    }
                    "Y" => match enemy_fromstr{ // need to draw
                        Ok(val) => Ok(val),
                        Err(msg) => return Err(msg),
                    }
                    "Z" => match enemy_fromstr{ // need to win
                        Ok(RockPaperScissors::Rock) => Ok(RockPaperScissors::Paper),
                        Ok(RockPaperScissors::Paper) => Ok(RockPaperScissors::Scissors),
                        Ok(RockPaperScissors::Scissors) => Ok(RockPaperScissors::Rock),
                        Err(msg) => return Err(msg),
                    }
                    str => Err(format!("Can't parse Rock/Paper/Scissors \"{str}\" for you")),
                };

                // What about the error?

                Ok(Strategy { enemy: enemy_fromstr.unwrap(), you_naive: you_naive_fromstr.unwrap(), you_actual : you_actual_fromstr.unwrap() })
            },
            None => Err(format!("Error parsing {s}")),
    }
    }
}

fn winner(you : RockPaperScissors, enemy : RockPaperScissors) -> GameResult {
    match you{
        RockPaperScissors::Rock => match enemy{
            RockPaperScissors::Rock => GameResult::Draw,
            RockPaperScissors::Paper => GameResult::Enemy,
            RockPaperScissors::Scissors => GameResult::You
        }
        RockPaperScissors::Paper => match enemy{
            RockPaperScissors::Rock => GameResult::You,
            RockPaperScissors::Paper => GameResult::Draw,
            RockPaperScissors::Scissors => GameResult::Enemy
        }
        RockPaperScissors::Scissors => match enemy{
            RockPaperScissors::Rock => GameResult::Enemy,
            RockPaperScissors::Paper => GameResult::You,
            RockPaperScissors::Scissors => GameResult::Draw
        }
    }
}

fn value(you : RockPaperScissors, enemy : RockPaperScissors) -> u32{
    let mut res = 0;
    res += match winner(you, enemy){
        GameResult::Enemy => 0,
        GameResult::Draw => 3,
        GameResult::You => 6,
    };
    res += match you {
        RockPaperScissors::Rock => 1,
        RockPaperScissors::Paper => 2,
        RockPaperScissors::Scissors => 3,
    };
    res
}

impl Strategy{
    fn res_naive(&self) -> u32{
        value(self.you_naive, self.enemy)
    }
    fn res_actual(&self) -> u32{
        value(self.you_actual, self.enemy)
    }

}

pub fn run(lines : Lines<BufReader<File>>) {
    let mut res_naive = 0;
    let mut res_actual = 0;
    for (_index, err_line) in lines.enumerate() {
        if let Ok(line) = err_line {
                let err_strategy = line.parse::<Strategy>();
                match err_strategy{
                    Ok(strategy) => {
                    //strategies.push(strategy);
                    res_naive += strategy.res_naive();
                    res_actual += strategy.res_actual();
                    },
                    Err(msg) =>
                        println!("Error: {msg}")
                }
        }
    }
    println!("total value (naive) : {}", res_naive);
    println!("total value (actual) : {}", res_actual);
}
