use std::collections::HashSet;
use std::{error::Error, fs};

fn get_priority(s1: &str, s2: &str) -> Option<u32> {
    let mut s2_hash = HashSet::new();
    for s2_char in s2.chars() {
        s2_hash.insert(s2_char);
    }
    for c1 in s1.chars() {
        if s2_hash.contains(&c1) {
            if c1 > 'a' {
                return Some(c1 as u8 as u32 - 96_u32);
            }
            if c1 > 'A' {
                return Some(c1 as u8 as u32 - 38_u32);
            }
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./inputs/03.txt")?;
    let mut priorities = Vec::new();
    for line in input.lines() {
        let (first_half, second_half) = line.split_at(line.len() / 2);
        match get_priority(first_half, second_half) {
            Some(val) => priorities.push(val),
            None => panic!("Couldn't find same item in rucksack"),
        }
    }
    dbg!(priorities.into_iter().sum::<u32>());
    Ok(())
}

#[cfg(test)]
#[test]
fn it_should_find_rucksack_priority() {
    assert_eq!(Some(16), get_priority("vJrwpWtwJgWr", "hcsFMMfFFhFp"));
    assert_eq!(Some(42), get_priority("PmmdzqPrV", "vPwwTWBwg"));
}
