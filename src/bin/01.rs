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
    let content = fs::read_to_string("./input.txt").unwrap();
    let (_, results) = many1(calories)(&content).unwrap();
    let mut elfs = vec![];

    for calories in results {
        elfs.push(calories.into_iter().fold(0, |acc, value| acc + value));
    }
    elfs.sort();
    elfs.reverse();

    dbg!(elfs.first());

    dbg!(elfs.into_iter().take(3).fold(0, |acc, value| acc + value));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = main(2, 2);
        assert_eq!(result, 4);
    }
}
