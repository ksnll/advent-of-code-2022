use std::{collections::HashSet, fs};

pub fn find_marker(chars: &Vec<char>) -> u32 {
    let mut iteration = 0_u32;
    for chars in chars.windows(14) {
        iteration = iteration + 1;
        dbg!(chars);
        match chars {
            [a, b, c, d, e, f, g, h, l, m, n, o, p, q] => {
                let hash = HashSet::from([a, b, c, d, e, f, g, h, l, m, n, o, p, q]);
                if hash.len() == 14 {
                    return iteration + 13;
                }
            }
            _ => panic!("Marker not found"),
        }
    }
    0
}

pub fn main() {
    let content = fs::read_to_string("./inputs/06.txt").unwrap();
    let chars = content.chars().collect::<Vec<char>>();
    let res = find_marker(&chars);
    dbg!(res);
}

#[cfg(test)]
#[test]
fn it_should_find_the_marker() {
    assert_eq!(
        19_u32,
        find_marker(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars().collect::<Vec<_>>())
    );
    assert_eq!(
        19_u32,
        find_marker(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars().collect::<Vec<_>>())
    );
}
