use std::{cell::RefCell, collections::HashSet, fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{space1, u32},
    IResult,
};

#[derive(Debug)]
struct Rope {
    x: i32,
    y: i32,

    tail_x: i32,
    tail_y: i32,
    visited_positions: RefCell<HashSet<String>>,
}

#[derive(Debug)]
enum Move {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

impl Rope {
    fn up(self: &mut Self, amount: u32) {
        for _ in 0..amount {
            self.y = self.y + 1;
            if self.y == self.tail_y + 2 {
                self.tail_y += 1;
                self.tail_x = self.x;
                self.visited_positions
                    .borrow_mut()
                    .insert(format!("{}-{}", self.tail_x, self.tail_y));
            }
        }
    }
    fn down(self: &mut Self, amount: u32) {
        for _ in 0..amount {
            self.y = self.y - 1;
            if self.y + 2 == self.tail_y {
                self.tail_y -= 1;
                self.tail_x = self.x;
                self.visited_positions
                    .borrow_mut()
                    .insert(format!("{}-{}", self.tail_x, self.tail_y));
            }
        }
    }
    fn right(self: &mut Self, amount: u32) {
        for _ in 0..amount {
            self.x = self.x + 1;
            if self.x == self.tail_x + 2 {
                self.tail_x += 1;
                self.tail_y = self.y;
                self.visited_positions
                    .borrow_mut()
                    .insert(format!("{}-{}", self.tail_x, self.tail_y));
            }
        }
    }
    fn left(self: &mut Self, amount: u32) {
        for _ in 0..amount {
            self.x = self.x - 1;
            if self.x + 2 == self.tail_x {
                self.tail_x -= 1;
                self.tail_y = self.y;
                self.visited_positions
                    .borrow_mut()
                    .insert(format!("{}-{}", self.tail_x, self.tail_y));
            }
        }
    }
}

fn move_command(input: &str) -> IResult<&str, Move> {
    let (input, move_tag) = alt((tag("U"), tag("L"), tag("D"), tag("R")))(input)?;
    let (input, _) = space1(input)?;
    let (input, amount) = u32(input)?;
    match move_tag {
        "U" => Ok((input, Move::Up(amount))),
        "L" => Ok((input, Move::Left(amount))),
        "D" => Ok((input, Move::Down(amount))),
        "R" => Ok((input, Move::Right(amount))),
        _ => panic!("Wrong input"),
    }
}

fn main() {
    let mut visited_positions = HashSet::new();
    visited_positions.insert("0-0".to_string());
    let mut head = Rope {
        x: 0,
        y: 0,
        tail_x: 0,
        tail_y: 0,
        visited_positions: RefCell::new(visited_positions),
    };
    let content = fs::read_to_string("./inputs/09.txt").unwrap();
    for line in content.lines() {
        let (_, command) = move_command(line).unwrap();
        match command {
            Move::Up(amount) => head.up(amount),
            Move::Down(amount) => head.down(amount),
            Move::Left(amount) => head.left(amount),
            Move::Right(amount) => head.right(amount),
        }
    }
    dbg!(&head.visited_positions.borrow().len());
}
