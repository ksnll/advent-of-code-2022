use std::{cell::RefCell, collections::HashSet, fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{space1, u32},
    IResult,
};

#[derive(Debug, Clone)]
struct Rope {
    x: i32,
    y: i32,
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
        }
    }
    fn down(self: &mut Self, amount: u32) {
        for _ in 0..amount {
            self.y = self.y - 1;
        }
    }
    fn right(self: &mut Self, amount: u32) {
        for _ in 0..amount {
            self.x = self.x + 1;
        }
    }
    fn left(self: &mut Self, amount: u32) {
        for _ in 0..amount {
            self.x = self.x - 1;
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

fn move_rope(rope: &mut Vec<RefCell<Rope>>) {
    for (prev_num, tail) in rope.iter().skip(1).enumerate() {
        let head = rope.get(prev_num).unwrap().borrow();
        let mut tail = tail.borrow_mut();

        if head.y == tail.y + 2 && head.x == tail.x + 2 {
            tail.y += 1;
            tail.x += 1;
            continue;
        }

        if head.y == tail.y - 2 && head.x == tail.x - 2 {
            tail.y -= 1;
            tail.x -= 1;
            continue;
        }

        if head.y == tail.y + 2 && head.x == tail.x - 2 {
            tail.y += 1;
            tail.x -= 1;
            continue;
        }

        if head.y == tail.y - 2 && head.x == tail.x + 2 {
            tail.y -= 1;
            tail.x += 1;
            continue;
        }

        if head.y == tail.y + 2 {
            tail.y += 1;
            tail.x = head.x;
            continue;
        }

        if head.y + 2 == tail.y {
            tail.y -= 1;
            tail.x = head.x;
            continue;
        }

        if head.x == tail.x + 2 {
            tail.x += 1;
            tail.y = head.y;
            continue;
        }

        if head.x + 2 == tail.x {
            tail.x -= 1;
            tail.y = head.y;
            continue;
        }
    }
}
fn main() {
    let mut visited_positions = HashSet::new();
    visited_positions.insert("0.0".to_string());
    let mut rope = vec![RefCell::new(Rope { x: 0, y: 0 }); 10];
    let content = fs::read_to_string("./inputs/09.txt").unwrap();
    for line in content.lines() {
        let (_, command) = move_command(line).unwrap();
        match command {
            Move::Up(amount) => {
                for _ in 0..amount {
                    rope.get_mut(0).unwrap().borrow_mut().up(1);
                    move_rope(&mut rope);
                    let tail = rope.last().unwrap().borrow();
                    visited_positions.insert(format!("{}.{}", tail.x, tail.y));
                }
            }

            Move::Down(amount) => {
                for _ in 0..amount {
                    rope.get_mut(0).unwrap().borrow_mut().down(1);
                    move_rope(&mut rope);
                    let tail = rope.last().unwrap().borrow();
                    visited_positions.insert(format!("{}.{}", tail.x, tail.y));
                }
            }
            Move::Left(amount) => {
                for _ in 0..amount {
                    rope.get_mut(0).unwrap().borrow_mut().left(1);
                    move_rope(&mut rope);
                    let tail = rope.last().unwrap().borrow();
                    visited_positions.insert(format!("{}.{}", tail.x, tail.y));
                }
            }
            Move::Right(amount) => {
                for _ in 0..amount {
                    rope.get_mut(0).unwrap().borrow_mut().right(1);
                    move_rope(&mut rope);
                    let tail = rope.last().unwrap().borrow();
                    visited_positions.insert(format!("{}.{}", tail.x, tail.y));
                }
            }
        }
    }
    dbg!(&rope);
    dbg!(visited_positions.len());
}
