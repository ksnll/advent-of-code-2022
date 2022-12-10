use colored::*;
use std::{cell::RefCell, fs};

#[derive(Debug)]
struct Tree {
    height: u32,
}

fn main() {
    let content = fs::read_to_string("./inputs/08.txt").unwrap();
    let mut tree_line = Vec::new();

    for (i, line) in content.lines().enumerate() {
        tree_line.push(Vec::new());
        for char in line.chars() {
            tree_line[i].push(RefCell::new(Tree {
                height: char.to_digit(10).unwrap(),
            }));
        }
    }

    let mut max_score = 0;

    for y in 0..tree_line.len() {
        for x in 0..tree_line.len() {
            let mut left_score = 0;
            let mut right_score = 0;
            let mut top_score = 0;
            let mut bottom_score = 0;
            let current_height = tree_line[y][x].borrow().height;

            while y.checked_sub(top_score + 1).is_some() {
                if let Tree { height } = *tree_line[y - top_score - 1][x].borrow() {
                    top_score += 1;
                    if height >= current_height {
                        break;
                    }
                }
            }
            while x.checked_sub(left_score + 1).is_some() {
                if let Tree { height } = *tree_line[y][x - left_score - 1].borrow() {
                    left_score += 1;
                    if height >= current_height {
                        break;
                    }
                }
            }
            while y + bottom_score + 1 < tree_line.len() {
                if let Tree { height } = *tree_line[y + bottom_score + 1][x].borrow() {
                    bottom_score += 1;
                    if height >= current_height {
                        break;
                    }
                }
            }

            while x + right_score + 1 < tree_line.len() {
                if let Tree { height } = *tree_line[y][x + right_score + 1].borrow() {
                    right_score += 1;
                    if height >= current_height {
                        break;
                    }
                }
            }
            let score = top_score * left_score * bottom_score * right_score;
            if score > max_score {
                max_score = score;
            }
        }
    }
    dbg!(max_score);
}
