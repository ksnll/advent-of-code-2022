use std::{borrow::BorrowMut, cell::RefCell, fs, ops::Deref, rc::Rc};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until},
    character::complete::{alpha1, char, newline, u32},
    combinator::{eof, rest},
    multi::many1,
    sequence::{preceded, terminated, tuple},
    IResult,
};

#[derive(Debug)]
enum LsNode {
    Directory(String, u32),
    File(String, u32),
}

#[derive(Debug)]
struct TreeNode {
    pub parent: Option<Rc<RefCell<TreeNode>>>,
    pub value: LsNode,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
}

#[derive(Debug)]
enum Command {
    Ls(Vec<LsNode>),
    Cd(String),
}

fn ls_node(input: &str) -> Option<LsNode> {
    let vec = input.split(' ').collect::<Vec<&str>>();
    match vec[0] {
        "" => None,
        "dir" => Some(LsNode::Directory(vec[1].to_string(), 0)),
        size @ _ => Some(LsNode::File(
            vec[1].to_string(),
            size.parse::<u32>().unwrap(),
        )),
    }
}

fn command(input: &str) -> IResult<&str, Command> {
    let (input, command) = alt((tag("ls"), tag("cd")))(input)?;
    let (input, arg) = alt((tag("\n"), preceded(tag(" "), take_till(|c| c == '\n'))))(input)?;
    let content = input.split("\n").collect::<Vec<&str>>();
    match command {
        "ls" => {
            let content = content
                .into_iter()
                .map(|content| ls_node(content))
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .collect();
            Ok((input, Command::Ls(content)))
        }
        "cd" => Ok((input, Command::Cd(String::from(arg)))),
        a @ _ => panic!("Command not found {a}"),
    }
}

fn main() {
    let mut tree = TreeNode {
        parent: None,
        value: LsNode::Directory("/".to_string(), 0),
        children: vec![],
    };
    let mut current_position = Rc::new(RefCell::new(tree));
    let head = current_position.clone();

    let input = fs::read_to_string("./inputs/07-test.txt").unwrap();
    let commands_and_results = input.split("$ ");
    for command_and_result in commands_and_results.skip(1) {
        let parsed_command = command(command_and_result);
        match parsed_command {
            Ok((_, Command::Cd(val))) => match val.as_str() {
                ".." => {
                    let parent = &current_position.borrow().parent.clone();
                    current_position = parent.clone().unwrap();
                }
                val => {
                    let parent = current_position.clone();
                    current_position = parent
                        .borrow()
                        .children
                        .iter()
                        .find(|x| match &x.clone().borrow().value {
                            LsNode::Directory(f, _) => *f == val,
                            LsNode::File(_, _) => false,
                        })
                        .unwrap()
                        .clone();
                }
            },
            Ok((_, Command::Ls(val))) => {
                for ls_node in val {
                    let child = Rc::new(RefCell::new(TreeNode {
                        parent: Some(current_position.clone()),
                        value: ls_node,
                        children: vec![],
                    }));
                    (*current_position)
                        .borrow_mut()
                        .children
                        .push(child.clone());
                }
            }
            Err(_) => panic!("Command not found"),
        }
    }

    fn visit_tree(tree: Rc<RefCell<TreeNode>>) {
        let mut tree = (*tree).borrow_mut();
        for i in &tree.children {
            visit_tree(i.clone());
        }
        match &tree.value {
            LsNode::Directory(val, _) => {
                let size = tree.children.clone().into_iter().fold(0, |acc, x| {
                    let x = x.borrow();
                    match x.value {
                        LsNode::File(_, file_size) => acc + file_size,
                        LsNode::Directory(_, file_size) => acc + file_size,
                    }
                });
                tree.value = LsNode::Directory(val.to_string(), size);
            }
            _ => (),
        }
    }
    visit_tree(head.clone());

    let x = u32::MAX;
    let sum = RefCell::new(x);
    fn print_tree(tree: Rc<RefCell<TreeNode>>, sum: &RefCell<u32>) {
        let mut tree = (*tree).borrow_mut();
        for i in &tree.children {
            print_tree(i.clone(), sum);
            let i = i.borrow();
            match &i.value {
                LsNode::Directory(name, size) => {
                    if size > &7052440u32 {
                        let mut sum = sum.borrow_mut();
                        if *size < *sum {
                            *sum = *size;
                        }
                    }
                }
                _ => (),
            }
        }
    }
    print_tree(head.clone(), &sum);
    dbg!(sum);
}
