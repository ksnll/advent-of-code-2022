use nom::{bytes::complete::tag, character::complete::u32, sequence::tuple, IResult};
use std::fs;

fn read_pair(input: &str) -> IResult<&str, ((u32, u32), (u32, u32))> {
    let (input, res) = tuple((u32, tag("-"), u32, tag(","), u32, tag("-"), u32))(input)?;
    Ok((input, ((res.0, res.2), (res.4, res.6))))
}

fn check_overlaps(s1: u32, e1: u32, s2: u32, e2: u32) -> bool {
    s1 >= s2 && s1 <= e2 || s2 >= s1 && s2 <= e1 || e1 >= s2 && e1 <= e2 || e2 >= s1 && e2 <= e1
}

fn main() {
    let content = fs::read_to_string("./inputs/04.txt").unwrap();
    let mut overlaps = 0;
    let mut total_overlaps = 0;
    for line in content.lines() {
        let (_, ((s1, e1), (s2, e2))) = read_pair(line).unwrap();
        if s1 >= s2 && e1 <= e2 || s2 >= s1 && e2 <= e1 {
            overlaps += 1;
        }
    }
    for line in content.lines() {
        let (_, ((s1, e1), (s2, e2))) = read_pair(line).unwrap();
        if check_overlaps(s1, e1, s2, e2) {
            dbg!((s1, e1, s2, e2));
            total_overlaps += 1;
        }
    }
    dbg!(overlaps);
    dbg!(total_overlaps);
}

#[cfg(test)]
#[test]
fn it_should_work() {
    assert_eq!(false, check_overlaps(2, 4, 6, 8));
    assert_eq!(false, check_overlaps(2, 3, 4, 5));
}
