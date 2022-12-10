use std::{collections::VecDeque, fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i32, space1},
    IResult,
};

enum Instruction {
    Noop,
    Addx(i32),
}
struct Execution {
    instruction: Instruction,
    cycles: u32,
}

fn addx(input: &str) -> IResult<&str, Instruction> {
    let (input, instruction) = tag("addx")(input)?;
    let (input, _) = space1(input)?;
    let (input, value) = i32(input)?;
    Ok((input, Instruction::Addx(value)))
}

fn noop(input: &str) -> IResult<&str, Instruction> {
    let (input, instruction) = tag("noop")(input)?;
    Ok((input, Instruction::Noop))
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, instruction) = alt((addx, noop))(input)?;
    Ok((input, instruction))
}
fn main() {
    let mut x: i32 = 1;
    let content = fs::read_to_string("./inputs/10-test.txt").unwrap();
    let mut executions = VecDeque::new();

    for line in content.lines() {
        let (input, instruction) = instruction(line).unwrap();
        match instruction {
            instruction @ Instruction::Noop => executions.push_back(Execution {
                instruction,
                cycles: 1,
            }),
            instruction @ Instruction::Addx(_) => executions.push_back(Execution {
                instruction,
                cycles: 2,
            }),
        }
    }
    let mut current_cycle = 1;
    let mut solution = 0;
    while let Some(ex) = executions.front_mut() {
        ex.cycles -= 1;
        if current_cycle == 20 || (current_cycle > 20 && ((current_cycle - 20) % 40) == 0) {
            solution += current_cycle * x;
        }
        if (ex.cycles == 0) {
            match ex.instruction {
                Instruction::Addx(val) => x += val,
                Instruction::Noop => (),
            }
            executions.pop_front();
        }
        current_cycle += 1;
    }
    dbg!(solution);
}

