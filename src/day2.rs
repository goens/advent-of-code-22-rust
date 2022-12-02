use std::io::{BufReader, Lines};
use std::fs::File;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug, PartialEq)]
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
    you : RockPaperScissors
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
                let you_fromstr = match you{
                    "X" => Ok(RockPaperScissors::Rock),
                    "Y" => Ok(RockPaperScissors::Paper),
                    "Z" => Ok(RockPaperScissors::Scissors),
                    str => Err(format!("Can't parse Rock/Paper/Scissors \"{str}\" for you")),
                };
                // What about the error?

                Ok(Strategy { enemy: enemy_fromstr.unwrap(), you: you_fromstr.unwrap() })
            },
            None => Err(format!("Error parsing {s}")),
    }
    }
}

impl Strategy{
    fn winner(&self) -> GameResult {
        match self.you{
            RockPaperScissors::Rock => match self.enemy{
                RockPaperScissors::Rock => GameResult::Draw,
                RockPaperScissors::Paper => GameResult::Enemy,
                RockPaperScissors::Scissors => GameResult::You
            }
            RockPaperScissors::Paper => match self.enemy{
                RockPaperScissors::Rock => GameResult::You,
                RockPaperScissors::Paper => GameResult::Draw,
                RockPaperScissors::Scissors => GameResult::Enemy
            }
            RockPaperScissors::Scissors => match self.enemy{
                RockPaperScissors::Rock => GameResult::Enemy,
                RockPaperScissors::Paper => GameResult::You,
                RockPaperScissors::Scissors => GameResult::Draw
            }
        }
    }
    fn value(&self) -> u32{
        let mut res = 0;
        res += match self.winner(){
            GameResult::Enemy => 0,
            GameResult::Draw => 3,
            GameResult::You => 6,
        };
        res += match self.you {
            RockPaperScissors::Rock => 1,
            RockPaperScissors::Paper => 2,
            RockPaperScissors::Scissors => 3,
        };
        res
    }

}

pub fn run(lines : Lines<BufReader<File>>) {
    let mut res = 0; // Vec::new();
    for (_index, err_line) in lines.enumerate() {
        if let Ok(line) = err_line {
                let err_strategy = line.parse::<Strategy>();
                match err_strategy{
                    Ok(strategy) => {
                    //strategies.push(strategy);
                    res += strategy.value();
                    },
                    Err(msg) =>
                        println!("Error: {msg}")
                }
        }
    }
    println!("total value : {}", res);
}
