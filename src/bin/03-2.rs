use itertools::Itertools;
use std::collections::HashSet;
use std::{error::Error, fs};

fn get_badge(s1: &str, s2: &str, s3: &str) -> char {
    let mut s1_hash = HashSet::new();
    for s1_char in s1.chars() {
        s1_hash.insert(s1_char);
    }
    let mut s2_hash = HashSet::new();
    for s2_char in s2.chars() {
        s2_hash.insert(s2_char);
    }
    let mut s3_hash = HashSet::new();
    for s3_char in s3.chars() {
        s3_hash.insert(s3_char);
    }

    for c4 in s1_hash.intersection(&s2_hash) {
        if s3_hash.contains(c4) {
            return *c4;
        }
    }
    panic!("Char not found")
}

fn get_priority(c: char) -> u32 {
    if c > 'a' {
        return c as u8 as u32 - 96_u32;
    }
    if c > 'A' {
        return c as u8 as u32 - 38_u32;
    }
    0
}

fn get_all_priorities(line1: &str, line2: &str, line3: &str) -> u32 {
    let mut priority = 0;
    let badge = get_badge(line1, line2, line3);
    priority += get_priority(badge);
    priority
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./inputs/03.txt")?;
    let mut priority = 0;
    for (l1, l2, l3) in input.lines().tuples() {
        priority += get_all_priorities(l1, l2, l3);
    }
    dbg!(priority);
    Ok(())
}

#[cfg(test)]
#[test]
fn it_should_find_rucksack_priority() {
    assert_eq!(
        18,
        get_all_priorities(
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg"
        )
    );
}
