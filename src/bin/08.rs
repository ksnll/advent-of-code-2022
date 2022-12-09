use colored::*;
use std::{cell::RefCell, fs};

#[derive(Debug)]
struct Tree {
    height: u32,
    visible: bool,
}

fn main() {
    let content = fs::read_to_string("./inputs/08.txt").unwrap();
    let mut tree_line = Vec::new();

    for (i, line) in content.lines().enumerate() {
        tree_line.push(Vec::new());
        for char in line.chars() {
            tree_line[i].push(RefCell::new(Tree {
                height: char.to_digit(10).unwrap(),
                visible: false,
            }));
        }
    }

    for (i, line) in tree_line.iter().enumerate() {
        let mut local_height = line[0].borrow().height;
        for (g, tree) in line.iter().enumerate() {
            if i == 0 || g == 0 || i == tree_line.len() - 1 || g == line.len() - 1 {
                tree.borrow_mut().visible = true;
                continue;
            }
            if local_height < tree.borrow().height {
                tree.borrow_mut().visible = true;
                local_height = tree.borrow().height;
            }
        }
    }

    for line in tree_line.iter() {
        let mut local_height = line[tree_line.len() - 1].borrow().height;
        for tree in line.iter().rev() {
            if local_height < tree.borrow().height {
                tree.borrow_mut().visible = true;
                local_height = tree.borrow().height;
            }
        }
    }

    for i in 0..tree_line.len() {
        let mut local_height = tree_line[0][i].borrow().height;
        for g in 0..tree_line.len() {
            if local_height < tree_line[g][i].borrow().height {
                tree_line[g][i].borrow_mut().visible = true;
                local_height = tree_line[g][i].borrow().height;
            }
        }
    }

    for i in 0..tree_line.len() {
        let mut local_height = tree_line[tree_line.len() - 1][i].borrow().height;
        for g in 0..tree_line.len() {
            if local_height < tree_line[tree_line.len() - g - 1][i].borrow().height {
                tree_line[tree_line.len() - g - 1][i].borrow_mut().visible = true;
                local_height = tree_line[tree_line.len() - g - 1][i].borrow().height;
            }
        }
    }

    let mut sum = 0;
    for line in &tree_line {
        for tree in line {
            print!(
                "{}",
                if tree.borrow().visible {
                    tree.borrow().height.to_string().yellow()
                } else {
                    tree.borrow().height.to_string().blue()
                }
            );
            if tree.borrow().visible {
                sum += 1;
            }
        }
        print!("\n");
    }
    dbg!(sum);
}
