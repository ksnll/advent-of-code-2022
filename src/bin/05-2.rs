use std::{collections::HashMap, fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, u32},
    combinator::eof,
    multi::many1,
    sequence::{delimited, tuple},
    IResult,
};

fn cargo_load(input: &str) -> IResult<&str, &str> {
    let (input, str) = delimited(tag("["), alphanumeric1, tag("]"))(input)?;
    Ok((input, str))
}

fn cargo_line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    let (input, res) = many1(tuple((alt((cargo_load, tag("   "))), alt((tag(" "), eof)))))(input)?;
    Ok((
        input,
        res.iter()
            .map(|(val, _)| match val {
                &"   " => None,
                char => Some(*char),
            })
            .collect(),
    ))
}

fn move_cargo(input: &str) -> IResult<&str, (u32, u32, u32)> {
    let (input, vals) = tuple((tag("move "), u32, tag(" from "), u32, tag(" to "), u32))(input)?;
    Ok((input, (vals.1, vals.3, vals.5)))
}

fn main() {
    let content = fs::read_to_string("./inputs/05.txt").unwrap();
    let mut cargo: HashMap<usize, String> = HashMap::new();
    for line in content.lines() {
        if let Ok((_, line)) = cargo_line(line) {
            for (index, value) in line.iter().enumerate() {
                let default_value = "".to_string();
                let cur_value = cargo.get(&index).unwrap_or(&default_value);
                cargo.insert(index, [value.unwrap_or(""), cur_value].concat());
            }
        }
        if let Ok((_, (what, from, to))) = move_cargo(line) {
            let cur_value = cargo.get(&(from as usize - 1_usize)).unwrap().clone();
            let dest_value = cargo.get(&(to as usize - 1_usize)).unwrap().clone();
            cargo.insert(
                from as usize - 1_usize,
                cur_value[0..cur_value.len() - what as usize].to_string(),
            );
            cargo.insert(
                to as usize - 1_usize,
                [
                    dest_value.to_owned(),
                    cur_value[cur_value.len() - what as usize..]
                        .chars()
                        .rev()
                        .collect(),
                ]
                .concat(),
            );
        }
    }
    let mut res = Vec::new();
    for i in 0..cargo.keys().len() {
        res.push(
            cargo
                .get(&i)
                .unwrap()
                .chars()
                .rev()
                .take(1)
                .collect::<String>(),
        );
    }
    dbg!(res.concat());
}
