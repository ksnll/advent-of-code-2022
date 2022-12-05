use anyhow::Result;
use nom::branch::alt;
use nom::character::complete::{line_ending, u32};
use nom::combinator::eof;
use nom::multi::many1;
use nom::IResult;
use std::fs;

fn calorie(input: &str) -> IResult<&str, u32> {
    let (input, calorie) = u32(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, calorie))
}

fn calories(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, calories) = many1(calorie)(input)?;
    let (input, _) = alt((line_ending, eof))(input)?;
    Ok((input, calories))
}
pub fn main() -> Result<()> {
    let content = fs::read_to_string("./inputs/01.txt").unwrap();
    let (_, results) = many1(calories)(&content).unwrap();
    let mut elfs: Vec<u32> = vec![];

    for calories in &results {
        elfs.push(calories.into_iter().sum());
    }

    elfs.sort();
    elfs.reverse();

    dbg!(elfs.first());

    dbg!(elfs.into_iter().take(3).sum::<u32>());
    Ok(())
}
