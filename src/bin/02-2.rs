use std::{error::Error, fs};

use nom::{branch::alt, bytes::complete::tag, IResult};

#[derive(Debug)]
enum Throw {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

enum GameResult {
    Loose = 0,
    Draw = 3,
    Win = 6,
}

fn game(input: &str) -> IResult<&str, (Throw, GameResult)> {
    let (input, throw1) = alt((tag("A"), tag("B"), tag("C")))(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, throw2) = alt((tag("Y"), tag("X"), tag("Z")))(input)?;
    let throw1 = match throw1 {
        "A" => Throw::Rock,
        "B" => Throw::Paper,
        "C" => Throw::Scissor,
        _ => panic!("Invalid input in throw1"),
    };
    let throw2 = match throw2 {
        "X" => GameResult::Loose,
        "Y" => GameResult::Draw,
        "Z" => GameResult::Win,
        _ => panic!("Invalid input in throw2"),
    };
    Ok((input, (throw1, throw2)))
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./inputs/02.txt")?;
    let lines = input.lines();
    let mut score = 0;
    for line in lines {
        match game(line) {
            Ok((_, (throw1, result))) => match (throw1, result) {
                (Throw::Rock, GameResult::Loose) => {
                    score += GameResult::Loose as u32 + Throw::Scissor as u32
                }
                (Throw::Rock, GameResult::Draw) => {
                    score += GameResult::Draw as u32 + Throw::Rock as u32
                }
                (Throw::Rock, GameResult::Win) => {
                    score += GameResult::Win as u32 + Throw::Paper as u32
                }
                (Throw::Paper, GameResult::Loose) => {
                    score += GameResult::Loose as u32 + Throw::Rock as u32
                }
                (Throw::Paper, GameResult::Draw) => {
                    score += GameResult::Draw as u32 + Throw::Paper as u32
                }
                (Throw::Paper, GameResult::Win) => {
                    score += GameResult::Win as u32 + Throw::Scissor as u32
                }
                (Throw::Scissor, GameResult::Loose) => {
                    score += GameResult::Loose as u32 + Throw::Paper as u32
                }
                (Throw::Scissor, GameResult::Draw) => {
                    score += GameResult::Draw as u32 + Throw::Scissor as u32
                }
                (Throw::Scissor, GameResult::Win) => {
                    score += GameResult::Win as u32 + Throw::Rock as u32
                }
            },
            Err(e) => panic!("Invalid game {e}"),
        }
    }
    println!("Score is {}", score);
    Ok(())
}
