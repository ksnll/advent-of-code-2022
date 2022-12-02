use std::{error::Error, fs};

use nom::{branch::alt, bytes::complete::tag, IResult};

#[derive(Debug)]
enum Throw {
    Rock,
    Paper,
    Scissor,
}

fn game(input: &str) -> IResult<&str, (Throw, Throw)> {
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
        "X" => Throw::Rock,
        "Y" => Throw::Paper,
        "Z" => Throw::Scissor,
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
            Ok((_, (throw1, throw2))) => match (throw1, throw2) {
                (Throw::Rock, Throw::Rock) => score += 3 + 1,
                (Throw::Paper, Throw::Paper) => score += 3 + 2,
                (Throw::Scissor, Throw::Scissor) => score += 3 + 3,
                (Throw::Paper, Throw::Rock) => score += 0 + 1,
                (Throw::Paper, Throw::Scissor) => score += 6 + 3,
                (Throw::Scissor, Throw::Rock) => score += 6 + 1,
                (Throw::Scissor, Throw::Paper) => score += 0 + 2,
                (Throw::Rock, Throw::Paper) => score += 6 + 2,
                (Throw::Rock, Throw::Scissor) => score += 0 + 3,
            },
            Err(e) => panic!("Invalid game {e}"),
        }
    }
    println!("Score is {}", score);
    Ok(())
}
